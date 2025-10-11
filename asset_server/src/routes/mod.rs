use axum::{routing::get, Router};

use crate::AppState;

mod chunk;
mod dev;
mod health;
mod model;

pub fn router() -> Router<AppState> {
    let app = Router::new()
        .nest("/dev", dev::router())
        .route("/health", get(health::health_check))
        .route("/model/{id}", get(model::get_model))
        .route("/chunk/{id}", get(chunk::get_chunk));

    return app;
}
