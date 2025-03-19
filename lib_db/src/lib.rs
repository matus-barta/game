use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn connect(db_url: String) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    Ok(pool)
}
