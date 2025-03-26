use std::{
    fs::{self, File},
    io::Write,
};

use crate::{fileupload::*, world_data::*, AppState};
use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "Service is healthy")
}

pub async fn chunk(Path(id): Path<u64>) -> impl IntoResponse {
    (StatusCode::OK, Json(get_chunk_info(id)))
}

pub async fn create_model(
    State(appState): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut file_name = String::new();
    let mut chunk_number = 0;
    let mut total_chunks = 0;
    let mut chunk_data = Vec::new();

    while let Some(field) = match multipart.next_field().await {
        Ok(f) => f,
        Err(err) => {
            tracing::error!("Error reading multipart field: {:?}", err);
            return StatusCode::BAD_REQUEST;
        }
    } {
        let field_name = field.name().unwrap_or_default().to_string();
        match field_name.as_str() {
            "fileName" => file_name = sanitize_filename(&field.text().await.unwrap_or_default()),
            "chunkNumber" => {
                chunk_number = field.text().await.unwrap_or_default().parse().unwrap_or(0)
            }
            "totalChunks" => {
                total_chunks = field.text().await.unwrap_or_default().parse().unwrap_or(0)
            }
            "chunk" => {
                chunk_data = field
                    .bytes()
                    .await
                    .unwrap_or_else(|_| Vec::new().into())
                    .to_vec()
            }
            _ => {}
        }
    }

    if file_name.is_empty() || chunk_data.is_empty() {
        return StatusCode::BAD_REQUEST;
    }

    let temp_dir = format!("./uploads/temp/{}", file_name);
    fs::create_dir_all(&temp_dir).unwrap_or_else(|_| {});

    let chunk_path = format!("{}/chunk_{}", temp_dir, chunk_number);
    let mut file = File::create(&chunk_path).unwrap();
    file.write_all(&chunk_data).unwrap();

    if is_upload_complete(&temp_dir, total_chunks) {
        assemble_file(&temp_dir, &file_name, total_chunks).unwrap();
        upload_to_s3_and_hash_and_store_in_db();
    }

    StatusCode::OK
}

fn upload_to_s3_and_hash_and_store_in_db() {}

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
