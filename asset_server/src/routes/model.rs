use crate::{
    models::{responses::Model, world_data::*},
    utils::error::internal_error,
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

/// GET -> /model/{id}
/// get model by hash (id) and presigned url to s3
pub async fn get_model(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Model>, (StatusCode, String)> {
    if id.len() != 64 {
        tracing::error!("Wrong id length: {:?}", id.len());
        Err((StatusCode::BAD_REQUEST, "Wrong id length".to_string()))?
    }

    let db_response = sqlx::query_as!(Asset, r#"SELECT id,name FROM "Asset" WHERE id = $1"#, id)
        .fetch_optional(&app_state.db_pool)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| "Not found".to_string())
        .map_err(|err| (StatusCode::NOT_FOUND, err))?;

    let presign_get = app_state
        .bucket
        .presign_get(format!("/{}", db_response.id), 300, None)
        .map_err(internal_error)?;

    let response = Model {
        url: presign_get,
        id: db_response.id,
        name: db_response.name,
    };

    Ok(Json(response))
}
