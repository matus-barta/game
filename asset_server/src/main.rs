use axum::{
    routing::{get, post},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;
use s3::{creds::Credentials, Bucket, BucketConfiguration, Region};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

mod fileupload;
mod routes;
mod utils;
mod world_data;

#[tokio::main]
async fn main() {
    let db_url = String::from("postgres://postgres:postgres@localhost/game");
    let s3_access_key = String::from("E2ebW9OMPAkW3FHldAuL");
    let s3_secret_key = String::from("UL16oJErzDCbZTHg6iqpjJomCtNliBY8oTy9mny7");
    let s3_api = String::from("http://localhost:9000 ");
    let s3_bucket = String::from("models");

    let region = Region::Custom {
        region: "eu-central-1".to_owned(),
        endpoint: s3_api,
    };
    let credentials =
        Credentials::new(Some(&s3_access_key), Some(&s3_secret_key), None, None, None)
            .expect("Wrong S3 credentials");

    let mut bucket = Bucket::new(&s3_bucket, region.clone(), credentials.clone())
        .unwrap() //idk what err is here
        .with_path_style();

    if !bucket.exists().await.unwrap() {
        //also idk here what error
        bucket = Bucket::create_with_path_style(
            &s3_bucket,
            region,
            credentials,
            BucketConfiguration::default(),
        )
        .await
        .expect("Could not create bucket")
        .bucket;
    }

    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

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
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer)
        .with_state(app_state);

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

#[derive(Clone)]
struct AppState {
    // that holds some api specific state
    db_pool: Pool<Postgres>,
    bucket: Box<Bucket>,
}
