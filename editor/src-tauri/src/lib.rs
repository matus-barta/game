use shared::requests::Asset;
use tauri_plugin_http::reqwest;

use crate::config::get_config;

mod config;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_model_list() -> Result<Vec<Asset>, String> {
    println!("get model list invoke");
    let config = get_config().map_err(|err| err.to_string())?;
    let res = reqwest::get(format!("{}/dev/model", config.asset_server_url))
        .await
        .map_err(|err| err.to_string())?;

    //println!("{:?}", res.text().await);

    let model = res
        .json::<Vec<Asset>>()
        .await
        .map_err(|err| err.to_string())?;
    println!("model: {:?}", model);

    Ok(model)
}

#[tauri::command]
async fn post_asset(path: String) -> Result<Vec<Asset>, String> {
    println!("post asset invoke");

    println!("FILE -> {}", path);

    let config = get_config().map_err(|err| err.to_string())?;

    let form = reqwest::multipart::Form::new()
        .file("", path)
        .await
        .map_err(|err| err.to_string())?;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/dev/model", config.asset_server_url))
        .multipart(form)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    match res.status().as_u16() {
        200..299 => {
            let model = res
                .json::<Vec<Asset>>()
                .await
                .map_err(|err| format!("Cant deserialize the data. Error: {}", err.to_string()))?;
            println!("model -> {:?}", model);

            Ok(model)
        }
        status_code => Err(format!(
            "Got {} status code, with msg \"{}\".",
            status_code,
            res.text().await.map_err(|err| format!(
                "Got error inside error (can't read response): {}, the status code is: {}",
                err.to_string(),
                status_code
            ))?
        )),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            let config = get_config().expect("Can't load config!");
            println!("Using server: {}", config.asset_server_url);
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                use tauri::Manager;

                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_model_list, post_asset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
