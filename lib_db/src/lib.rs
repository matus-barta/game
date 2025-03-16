use diesel::prelude::*;

pub mod models;
pub mod schema;

pub fn connect(db_url: String) -> PgConnection {
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
