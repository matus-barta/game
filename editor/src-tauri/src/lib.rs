use shared::requests::Model;
use tauri_plugin_http::reqwest;

use crate::config::get_config;

mod config;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_model_list() -> Result<Model, String> {
    println!("get model list invoke");
    let config = get_config().map_err(|err| err.to_string())?;
    println!("config done");
    let res = reqwest::get(config.asset_server_url)
        .await
        .map_err(|err| err.to_string())?;
    println!("{:?}", res.text().await);

    let model: Model = {
        Model {
            url: "".to_string(),
            id: "".to_string(),
            name: "".to_string(),
        }
    }; // = res.json::<Model>().await.map_err(|err| err.to_string())?;
    println!("model");

    return Ok(model);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                use tauri::Manager;

                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_model_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
