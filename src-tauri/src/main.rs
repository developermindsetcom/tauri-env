// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_env(name: &str) -> String {
    std::env::var(String::from(name)).unwrap_or(String::from(""))
}

fn main() {
    // dotenv::load().ok();

    if cfg!(debug_assertions) {
        dotenv::from_filename(".env.development").unwrap().load();
    }else{
        // loads from the root of the `tauri build` command
        // dotenv::from_filename(".env.production").unwrap().load();

        // loads inside the source code relative to this file
        let prod_env = include_str!("../../.env.production");
        // println!("prod_env {}", prod_env);
        let result = dotenv::from_read(prod_env.as_bytes()).unwrap();
        result.load();
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // .setup(|_app| {
            // use tauri::Manager as _;
            // let main = app.handle().get_window("main").unwrap();
            // main.open_devtools();

            // Ok(())
        // })
        .invoke_handler(tauri::generate_handler![get_env])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
