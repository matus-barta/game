use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use axum_prometheus::PrometheusMetricLayer;
use serde::Serialize;
use serde_repr::Serialize_repr;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

mod routes;
mod utils;
mod world_data;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db_url = String::from("postgres://postgres:postgres@localhost/game");
    let db = lib_db::connect(db_url)
        .await
        .expect("Unable to connect to DB");

    let (prometheus_layer, metrics_handle) = PrometheusMetricLayer::pair();

    // build our application with a route
    let app = Router::new()
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer)
        .with_state(db)
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/metrics", get(|| async move { metrics_handle.render() }))
        .route("/chunk/{id}", get(routes::chunk))
        .route("/health", get(routes::health));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Could not initialize TcpListener");

    tracing::info!(
        "listening on {}",
        listener
            .local_addr()
            .expect("Could not convert listener to local address")
    );
    axum::serve(listener, app)
        .await
        .expect("Could not successfully create server");
}
