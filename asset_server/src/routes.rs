use crate::{utils::internal_error, world_data::*, AppState};
use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sha2::{Digest, Sha256};

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "Service is healthy")
}

pub async fn chunk(Path(id): Path<u64>) -> impl IntoResponse {
    (StatusCode::OK, Json(get_chunk_info(id)))
}

pub async fn get_model(State(app_state): State<AppState>, id: String) -> impl IntoResponse {
    if id.len() != 64 {
        return (StatusCode::BAD_REQUEST, "Wrong id length");
    }

    (StatusCode::OK, "OK")
}

pub async fn create_model(
    State(app_state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Response, (StatusCode, String)> {
    let mut response_models: Vec<Model> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        let name = field.name().unwrap_or_default().to_string();
        let file_name = field.file_name().unwrap_or_default().to_string();
        let content_type = field.content_type().unwrap_or_default().to_string();
        let data = field.bytes().await.unwrap_or_default();

        //TODO: Sanitize filename

        if file_name.is_empty() || data.is_empty() || content_type.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Filename, data or content_type is empty".to_string(),
            ))?;
        }
        if content_type != "model/gltf-binary" {
            return Err((StatusCode::BAD_REQUEST, "Wrong content type".to_string()))?;
        }

        let hash = Sha256::digest(&data);

        let _response_data = app_state
            .bucket
            .put_object_with_content_type(format!("/{:X}", hash), &data, &content_type)
            .await
            .map_err(internal_error)?;

        let db_response = sqlx::query_as!(
            Model,
            r#"INSERT INTO models VALUES ($1, $2) RETURNING id, model_name"#,
            format!("{:X}", &hash),
            &file_name
        )
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(internal_error)?;

        response_models.push(db_response);

        tracing::debug!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes with hash {:x}",
            data.len(),
            hash
        );
    }
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(
            serde_json::to_string(&response_models).map_err(internal_error)?,
        ))
        .map_err(internal_error)?)
}

fn get_chunk_info(id: u64) -> Chunk<'static> {
    //TODO: Search DB for chunk and return it
    Chunk {
        id,
        terrain_id: "floor.glb",
        world_objects: Some(vec![WorldObject {
            id: 222,
            model_id: "house.glb",
            position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            placables: None,
        }]),
        placables: Some(vec![
            Placable {
                id: 78768,
                light_source: None,
                model_id: Some("TwistedTree_1.glb"),
                position: Vec3 {
                    x: 10.0,
                    y: 0.0,
                    z: 7.0,
                },
                rotation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            Placable {
                id: 78768,
                light_source: None,
                model_id: Some("Plant_1_Big.glb"),
                position: Vec3 {
                    x: -2.2,
                    y: 0.0,
                    z: 4.0,
                },
                rotation: Vec3 {
                    x: 0.0,
                    y: 52.0,
                    z: 0.0,
                },
            },
            Placable {
                id: 78768,
                light_source: None,
                model_id: Some("Pine_1.glb"),
                position: Vec3 {
                    x: -11.0,
                    y: 0.0,
                    z: 6.5,
                },
                rotation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            Placable {
                id: 78768,
                light_source: None,
                model_id: Some("Flower_3_Group.glb"),
                position: Vec3 {
                    x: 2.5,
                    y: 0.0,
                    z: 3.3,
                },
                rotation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            Placable {
                id: 78768,
                light_source: Some(LightSource {
                    light_type: LightType::OMNI,
                    color: Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                    },
                    energy: 2.0,
                    size: 1.0,
                }),
                model_id: None,
                position: Vec3 {
                    x: 0.0,
                    y: 2.0,
                    z: 0.0,
                },
                rotation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
        ]),
    }
}
