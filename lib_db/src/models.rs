use diesel::prelude::*;

use crate::schema::{
    chunks, models, placables,
    sql_types::{LightSource, Vec3},
    world_objects,
};

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Model))]
#[diesel(table_name = chunks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chunk {
    pub id: i64,
    pub model_id: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Model))]
#[diesel(table_name = world_objects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorldObject {
    pub id: i64,
    pub model_id: String,
    pub position: Vec3,
    pub rotation: Vec3,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Chunk, foreign_key = parent_id))]
#[diesel(belongs_to(WorldObject, foreign_key = parent_id))]
#[diesel(belongs_to(Model))]
#[diesel(table_name = placables)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Placable {
    pub id: i64,
    pub model_id: String,
    pub position: Vec3,
    pub rotation: Vec3,
    pub light_source: Option<LightSource>,
    pub parent_id: i64,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = models)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Model {
    pub id: String,
    pub model_name: String,
}
