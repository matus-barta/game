use std::env;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;
use s3::{creds::Credentials, Bucket, BucketConfiguration, Region};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

mod responses;
mod routes;
mod utils;
mod world_data;

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

    let bucket_name = String::from("models");
    let region = Region::Custom {
        region: "eu-central-1".to_owned(),
        endpoint: s3_api.to_owned(),
    };

    let credentials =
        Credentials::new(Some(&s3_access_key), Some(&s3_secret_key), None, None, None).unwrap();

    let mut bucket = Bucket::new(&bucket_name, region.clone(), credentials.clone())
        .unwrap()
        .with_path_style();

    if !bucket.exists().await.unwrap() {
        bucket = Bucket::create_with_path_style(
            &bucket_name,
            region,
            credentials,
            BucketConfiguration::default(),
        )
        .await
        .unwrap()
        .bucket;
    }

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("Unable to connect to DB");

    let app_state = AppState {
        db_pool: db,
        bucket,
    };

    let (prometheus_layer, metrics_handle) = PrometheusMetricLayer::pair();

    // build our application with a route
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/metrics", get(|| async move { metrics_handle.render() }))
        .route("/chunk/{id}", get(routes::chunk))
        .route("/health", get(routes::health))
        .route("/model", post(routes::create_model))
        .route("/model/{id}", get(routes::get_model))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250mb */
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
