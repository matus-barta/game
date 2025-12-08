use redis::aio::MultiplexedConnection;

pub async fn init_redis(redis_url: String) -> MultiplexedConnection {
    let client = redis::Client::open(redis_url).expect("unable to open Redis connection");
    let conn = client
        .get_multiplexed_async_connection()
        .await
        .expect("cant get Multiplexed connection");

    tracing::info!("Connected to Redis");

    return conn;
}
