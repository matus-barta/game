use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a route
    let app = Router::new()
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
        world_objects: vec![WorldObject {
            id: 222,
            model_id: "house.glb",
            transform: Vec3 {
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
        }],
        placables: Some(vec![Placable {
            id: 78768,
            model_id: "placable2.glb",
            transform: Vec3 {
                x: 4.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }]),
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
    world_objects: Vec<WorldObject<'a>>,
    placables: Option<Vec<Placable<'a>>>,
}

#[derive(Serialize)]
struct WorldObject<'a> {
    id: u64,
    model_id: &'a str, //TODO: make it hash
    transform: Vec3,
    rotation: Vec3,
    placables: Option<Vec<Placable<'a>>>,
}

#[derive(Serialize)]
struct Placable<'a> {
    id: u64,
    model_id: &'a str, //TODO: Make it hash
    transform: Vec3,
    rotation: Vec3,
}

#[derive(Serialize)]
struct Vec3 {
    //TODO: impl some 3d lib
    x: f32,
    y: f32,
    z: f32,
}
