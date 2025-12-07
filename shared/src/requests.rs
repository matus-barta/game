use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub id: String,
    pub name: String,
}
