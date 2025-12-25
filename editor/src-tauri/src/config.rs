use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub asset_server_url: String,
}

pub fn get_config() -> Result<Config, serde_json::Error> {
    #[cfg(debug_assertions)]
    let config = include_str!("../../local.config.json");

    #[cfg(not(debug_assertions))]
    let config = include_str!("../../config.json");

    serde_json::from_str::<Config>(config)
}
