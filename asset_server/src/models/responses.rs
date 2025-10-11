use serde::Serialize;

#[derive(Serialize)]
pub struct Model {
    pub url: String,
    pub id: String,
    pub name: String,
}
