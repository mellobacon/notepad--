pub mod menu;

use menu::Menu;
use tauri::{menu::MenuBuilder, Emitter};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            let m = Menu::new(app);
            let menu = MenuBuilder::new(app)
                .items(&[
                    &m.file_menu(),
                    &m.edit_menu(),
                    &m.format_menu(),
                    &m.view_menu(),
                    &m.help_menu(),
                ])
                .build()?;
            app.set_menu(menu)?;

            app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
                match event.id().0.as_str() {
                    "status_bar" => {
                        // sue me
                        let x = app_handle.menu().unwrap().get("view").unwrap().as_submenu().unwrap().items().unwrap().last().unwrap().as_check_menuitem().unwrap().is_checked().unwrap();
                        app_handle.emit("status_bar", x).unwrap();
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

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
