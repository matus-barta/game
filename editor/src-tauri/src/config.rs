use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub asset_server_url: String,
}

pub fn get_config() -> Result<Config, serde_json::Error> {
    const DEBUG: bool = true;

    let config = if cfg!(debug_assertions) || !DEBUG {
        include_str!("../../config.json")
    } else {
        include_str!("../../local.config.json")
    };

    serde_json::from_str::<Config>(config)
}
