use crate::models::world_data::*;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

/// GET -> /chunk/{id}
/// returns JSON of chunk data
pub async fn get_chunk(Path(id): Path<u64>) -> impl IntoResponse {
    (StatusCode::OK, Json(get_chunk_info(id)))
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
