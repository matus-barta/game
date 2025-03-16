use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use serde_repr::Serialize_repr;
use tower_http::{compression::CompressionLayer, services::ServeDir};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a route
    let app = Router::new()
        .layer(CompressionLayer::new())
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/chunk/{id}", get(get_chunk_info));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_chunk_info(Path(id): Path<u64>) -> impl IntoResponse {
    //TODO: Search DB for chunk and return it
    let chunk = Chunk {
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
    };

    tracing::info!("Sent Chunk Info");
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(chunk))
}

#[derive(Serialize)]
struct Chunk<'a> {
    id: u64,
    terrain_id: &'a str,
    world_objects: Option<Vec<WorldObject<'a>>>,
    placables: Option<Vec<Placable<'a>>>,
}

#[derive(Serialize)]
struct WorldObject<'a> {
    id: u64,
    model_id: &'a str, //TODO: make it hash
    position: Vec3,
    rotation: Vec3,
    placables: Option<Vec<Placable<'a>>>,
}

#[derive(Serialize)]
struct Placable<'a> {
    id: u64,
    model_id: Option<&'a str>, //TODO: Make it hash
    position: Vec3,
    rotation: Vec3,
    light_source: Option<LightSource>,
}

#[derive(Serialize)]
struct LightSource {
    light_type: LightType,
    color: Color,
    energy: f32,
    size: f32,
}

#[derive(Serialize_repr)]
#[repr(u8)]
enum LightType {
    SPOT = 0,
    OMNI = 1,
}

#[derive(Serialize)]
struct Color {
    //TODO: impl some 3d lib
    r: f32,
    g: f32,
    b: f32,
}
#[derive(Serialize)]
struct Vec3 {
    //TODO: impl some 3d lib
    x: f32,
    y: f32,
    z: f32,
}
