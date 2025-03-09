use bevy_http_client::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Chunk {
    id: u64,
    terrain_id: String,
    world_objects: Vec<WorldObject>,
    placables: Option<Vec<Placable>>,
}

#[derive(Deserialize, Debug)]
struct WorldObject {
    id: u64,
    model_id: String, //TODO: make it hash
    transform: Vec3,
    rotation: Vec3,
    placables: Option<Vec<Placable>>,
}

#[derive(Deserialize, Debug)]
struct Placable {
    id: u64,
    model_id: String, //TODO: Make it hash
    transform: Vec3,
    rotation: Vec3,
}

#[derive(Deserialize, Debug)]
struct Vec3 {
    //TODO: impl some 3d lib
    x: f32,
    y: f32,
    z: f32,
}

use bevy::prelude::*;

pub struct WebAssetsPlugin;

impl Plugin for WebAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HttpClientPlugin)
            .add_systems(Update, handle_response)
            .add_systems(Startup, send_request);
        app.register_request_type::<Chunk>();
    }
}

fn send_request(mut ev_request: EventWriter<TypedRequest<Chunk>>) {
    ev_request.send(
        HttpClient::new()
            .get("http://127.0.0.1:3000/chunk/1")
            .with_type::<Chunk>(),
    );
}

fn handle_response(mut ev_response: EventReader<TypedResponse<Chunk>>) {
    for response in ev_response.read() {
        println!("response: {:?}", response);
    }
}
