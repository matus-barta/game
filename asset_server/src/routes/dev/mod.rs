use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

mod model;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/model", post(model::create_model))
        .route("/model", get(model::get_models))
}
