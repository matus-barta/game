use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Serialize)]
pub struct Chunk<'a> {
    pub id: u64,
    pub terrain_id: &'a str,
    pub world_objects: Option<Vec<WorldObject<'a>>>,
    pub placables: Option<Vec<Placable<'a>>>,
}

#[derive(Serialize)]
pub struct WorldObject<'a> {
    pub id: u64,
    pub model_id: &'a str, //TODO: make it hash
    pub position: Vec3,
    pub rotation: Vec3,
    pub placables: Option<Vec<Placable<'a>>>,
}

#[derive(Serialize)]
pub struct Placable<'a> {
    pub id: u64,
    pub model_id: Option<&'a str>, //TODO: Make it hash
    pub position: Vec3,
    pub rotation: Vec3,
    pub light_source: Option<LightSource>,
}

#[derive(Serialize)]
pub struct LightSource {
    pub light_type: LightType,
    pub color: Color,
    pub energy: f32,
    pub size: f32,
}

#[derive(Serialize)]
pub struct Model {
    pub id: u64,
    pub asset_id: String,
    pub model_name: String,
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum LightType {
    SPOT = 0,
    OMNI = 1,
}

#[derive(Serialize)]
pub struct Color {
    //TODO: impl some 3d lib
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
#[derive(Serialize)]
pub struct Vec3 {
    //TODO: impl some 3d lib
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
