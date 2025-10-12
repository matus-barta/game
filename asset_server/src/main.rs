use std::env;

use axum::{extract::DefaultBodyLimit, routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use s3::Bucket;
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

mod helpers;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("rustls error");

    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let _ = dotenvy::dotenv(); //we try to load .env file we don't care if it fails because is expected if env file is missing the variables itself are initialized

    let server_ip_port = env::var("SERVER_IP_PORT").unwrap_or("0.0.0.0:3000".into());
    let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL env var");

    let s3_access_key = env::var("S3_ACCESS_KEY").expect("Missing S3_ACCESS_KEY env var");
    let s3_secret_key = env::var("S3_SECRET_KEY").expect("Missing S3_SECRET_KEY env var");
    let s3_api = env::var("S3_API_URL").expect("Missing S3_API_URL env var");

    let app_state = AppState {
        db_pool: helpers::pg::init_pg(db_url).await,
        bucket: helpers::s3::init_bucket(
            s3_access_key,
            s3_secret_key,
            s3_api,
            "assets".to_string(),
        )
        .await,
    };

    let (prometheus_layer, metrics_handle) = PrometheusMetricLayer::pair();

    // build our application with a route
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/metrics", get(|| async move { metrics_handle.render() }))
        .merge(routes::router())
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer)
        .layer(DefaultBodyLimit::disable())
        .layer(CorsLayer::permissive())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250MiB */
        ))
        .with_state(app_state);

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(server_ip_port)
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

#[derive(Clone)]
struct AppState {
    // that holds some api specific state
    db_pool: Pool<Postgres>,
    bucket: Bucket,
}
