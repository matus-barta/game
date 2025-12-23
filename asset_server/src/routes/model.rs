use crate::{
    helpers::redis::cache_error, models::world_data::*, utils::error::internal_error, AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use metrics::counter;
use redis::AsyncTypedCommands;

const TTL: u32 = 300;

/// GET -> /model/{id}
/// get model by hash (id) and presigned url to s3
/// CACHED route
pub async fn get_model(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<shared::requests::Model>, (StatusCode, String)> {
    //validate length
    if id.len() != 64 {
        tracing::error!("Wrong id length: {:?}", id.len());
        Err((StatusCode::BAD_REQUEST, "Wrong id length".to_string()))?
    }

    let mut redis = app_state.redis.lock().await;
    let response = match get_cached(redis.get(&id).await, &id) {
        //return cached data
        Some(data) => data,
        //read from db and s3
        None => {
            let db_response =
                sqlx::query_as!(Asset, r#"SELECT id,name FROM "Asset" WHERE id = $1"#, id)
                    .fetch_optional(&app_state.db_pool)
                    .await
                    .map_err(internal_error)?
                    .ok_or_else(|| "Not found".to_string())
                    .map_err(|err| (StatusCode::NOT_FOUND, err))?;

            //validate the file exists
            let _ = app_state
                .bucket
                .get_object(format!("/{}", db_response.id))
                .await
                .map_err(internal_error)?;

            let presign_get = app_state
                .bucket
                .presign_get(format!("/{}", db_response.id), TTL, None)
                .map_err(internal_error)?;

            let response = shared::requests::Model {
                url: presign_get,
                id: db_response.id,
                name: db_response.name,
            };

            //cache the response
            match serde_json::to_string(&response) {
                Ok(json) => match redis.set_ex(&id, json, TTL.into()).await {
                    Ok(_) => tracing::debug!("Set cached key: {}", &id),
                    Err(e) => cache_error(e),
                },
                Err(e) => cache_error(e),
            };

            //return the response
            response
        }
    };

    Ok(Json(response))
}

fn get_cached(
    redis_response: Result<Option<String>, redis::RedisError>,
    id: &String,
) -> Option<shared::requests::Model> {
    match redis_response {
        Ok(data) => match data {
            Some(data) => {
                tracing::debug!("Cache hit - key: {} / value: {:?}", &id, &data);
                counter!("cache hit").increment(1);
                match serde_json::from_str(&data) {
                    //FIXME: ??? we are deserializing the data and the serializing again,
                    Ok(model) => Some(model), //the deserialization makes sure we get correct data but it may be some slowdown because it it
                    Err(e) => {
                        cache_error(e);
                        None
                    }
                }
            }
            None => {
                tracing::debug!("Cache miss - key: {}", &id);
                counter!("cache miss").increment(1);
                None
            }
        },
        Err(e) => {
            cache_error(e);
            None
        }
    }
}
