// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "light_source"))]
    pub struct LightSource;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "vec3"))]
    pub struct Vec3;
}

diesel::table! {
    chunks (id) {
        id -> Int8,
        #[max_length = 64]
        model_id -> Varchar,
    }
}

diesel::table! {
    models (id) {
        #[max_length = 64]
        id -> Varchar,
        model_name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Vec3;
    use super::sql_types::LightSource;

    placables (id) {
        id -> Int8,
        #[max_length = 64]
        model_id -> Varchar,
        position -> Vec3,
        rotation -> Vec3,
        light_source -> Nullable<LightSource>,
        parent_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Vec3;

    world_objects (id) {
        id -> Int8,
        #[max_length = 64]
        model_id -> Varchar,
        position -> Vec3,
        rotation -> Vec3,
    }
}

diesel::joinable!(placables -> chunks (parent_id));
diesel::joinable!(placables -> models (model_id));
diesel::joinable!(placables -> world_objects (parent_id));
diesel::joinable!(world_objects -> models (model_id));

diesel::allow_tables_to_appear_in_same_query!(chunks, models, placables, world_objects,);
