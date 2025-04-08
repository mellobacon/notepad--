pub mod menu;

use log::info;
use menu::Menu;
use tauri_plugin_store::StoreExt;
use std::{collections::HashMap, time::Duration};
use tauri::{menu::MenuBuilder, App, Emitter, Manager};

fn set_log_config(app: &mut App) {
    if cfg!(debug_assertions) {
        app.handle()
            .plugin(
                tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .build(),
            )
            .unwrap();
    }
}

fn load_default_settings(app: &mut App) {
    info!("Loading settings from config...");

    let default_settings = serde_json::json!(
        {
            "show_statusbar": true,
            "word_wrap": false
        }
    );

    let mut defaults = HashMap::new();
    for settings in default_settings.as_object().unwrap() {
        defaults
            .entry(settings.0.clone())
            .or_insert_with(|| settings.1.clone());
    }

    let store = if app.path().app_data_dir().unwrap().join("config.json").exists() {
        info!("Config found. Loaded");
        app.store("config.json").unwrap()
    }
    else {
        info!("No config found. Created new one");
        app.store_builder("config.json").defaults(defaults).auto_save(Duration::from_secs(1)).build().unwrap()
    };
    app.manage(store);
}

fn load_menus(app: &mut App) {
    let m = Menu::new(app);
    let menu = MenuBuilder::new(app)
        .items(&[
            &m.file_menu(),
            &m.edit_menu(),
            &m.format_menu(),
            &m.view_menu(),
            &m.help_menu(),
        ])
        .build()
        .unwrap();
    app.set_menu(menu).unwrap();

    app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
        match event.id().0.as_str() {
            "status_bar" => {
                app_handle.emit("status_bar", ()).unwrap();
            }
            "backspace" => {
                app_handle.emit("backspace", ()).unwrap();
            }
            "cursor_left" => {
                app_handle.emit("cursor", -1).unwrap();
            }
            "cursor_right" => {
                app_handle.emit("cursor", 1).unwrap();
            }
            "word_wrap" => {
                app_handle.emit("word_wrap", ()).unwrap();
            }
            "new_line" => {
                app_handle.emit("new_line", ()).unwrap();
            }
            _ => {
                if let Some(c) = event.id().0.strip_prefix("insert_") {
                    if c.len() == 1 {
                        let c = c.chars().next().unwrap();
                        if (' '..='~').contains(&c) {
                            app_handle.emit(&format!("insert_char"), &c).unwrap();
                            return;
                        }
                    }
                }
                println!("menu event: {:?}", event);
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            set_log_config(app);
            load_default_settings(app);
            load_menus(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
