use std::sync::Arc;

use tauri::{menu::{AboutMetadataBuilder, CheckMenuItem, IsMenuItem, MenuItem, SubmenuBuilder}, App, Manager, Wry};
use tauri_plugin_store::Store;

pub struct Menu<'a> {
    app: &'a App
}

impl<'a> Menu<'a> {
    fn get_letters(&self, start: char, end: char) -> Vec<Box<dyn IsMenuItem<Wry>>> {
        let letters: Vec<Box<dyn IsMenuItem<Wry>>> = (start..=end)
        .map(|i| {
            Box::new(
                MenuItem::with_id(
                    self.app,
                    format!("insert_{}", i as u8 as char),
                    format!("{}", i as u8 as char),
                    true,
                    Some(""),
                )
                .unwrap(),
            ) as Box<dyn IsMenuItem<Wry>>
        })
        .collect();
        letters
    }
    fn get_numbers(&self) -> Vec<Box<dyn IsMenuItem<Wry>>> {
        let letters: Vec<Box<dyn IsMenuItem<Wry>>> = (48..=57)
        .map(|i| {
            Box::new(
                MenuItem::with_id(
                    self.app,
                    format!("insert_{}", i as u8 as char),
                    format!("{}", i as u8 as char),
                    true,
                    Some(""),
                )
                .unwrap(),
            ) as Box<dyn IsMenuItem<Wry>>
        })
        .collect();
        letters
    }
    fn get_chars(&self) -> Vec<Box<dyn IsMenuItem<Wry>>> {
    let ranges = [' '..='/', ':'..='@', '['..='`', '{'..='~'];
    ranges
        .iter()
        .flat_map(|range| range.clone())
        .map(|i| {
            Box::new(
                MenuItem::with_id(
                    self.app,
                    format!("insert_{}", i as u8 as char),
                    format!("&{}", i as u8 as char),
                    true,
                    Some(""),
                )
                .unwrap(),
            ) as Box<dyn IsMenuItem<Wry>>
        })
        .collect()
    }
    fn get_all_ascii(&self) -> Vec<Box<dyn IsMenuItem<Wry>>> {
        let letters: Vec<Box<dyn IsMenuItem<Wry>>> = (32..=126)
        .map(|i| {
            Box::new(
                MenuItem::with_id(
                    self.app,
                    format!("insert_{}", i as u8 as char),
                    format!("&{}", i as u8 as char),
                    true,
                    Some(""),
                )
                .unwrap(),
            ) as Box<dyn IsMenuItem<Wry>>
        })
        .collect();
        letters
    }

    pub fn new (app: &'a App) -> Self {
        Self { app }
    }
    pub fn file_menu(&self) -> tauri::menu::Submenu<Wry> {
        let new = MenuItem::with_id(
                self.app,
                "new",
                "New",
                true,
                Some("Ctrl+N"),
            ).unwrap();
        let new_window = MenuItem::with_id(
                self.app,
                "new_window",
                "New Window",
                false,
                Some("Ctrl+Shift+N"),
            ).unwrap();
        let open = MenuItem::with_id(
                self.app,
                "open",
                "Open",
                true,
                Some("Ctrl+O"),
            ).unwrap();
        let print = MenuItem::with_id(
            self.app,
            "print",
            "Print",
            true,
            Some("Ctrl+O"),
        ).unwrap();
        SubmenuBuilder::with_id(self.app, "file", "File")
            .item(&new)
            .item(&new_window)
            .item(&open)
            .item(&print)
            .separator()
            .quit()
            .build().unwrap()
    }
    pub fn edit_menu(&self) -> tauri::menu::Submenu<Wry> {
        let lowercase_letters = self.get_letters('a', 'z');
        let uppercase_letters = self.get_letters('A', 'Z');
        let numbers = self.get_numbers();
        let chars = self.get_chars();
        let all = self.get_all_ascii();
        let lowercase_letters_list: Vec<&dyn IsMenuItem<Wry>> = lowercase_letters.iter().map(|item| &**item).collect();
        let uppercase_letters_list: Vec<&dyn IsMenuItem<Wry>> = uppercase_letters.iter().map(|item| &**item).collect();
        let numbers_list: Vec<&dyn IsMenuItem<Wry>> = numbers.iter().map(|item| &**item).collect();
        let chars_list: Vec<&dyn IsMenuItem<Wry>> = chars.iter().map(|item| &**item).collect();
        let all_list: Vec<&dyn IsMenuItem<Wry>> = all.iter().map(|item| &**item).collect();

        let item = MenuItem::with_id(
                self.app,
                "delete",
                "Del",
                true,
                Some("Delete"),
            ).unwrap();
        let paste = MenuItem::with_id(
                self.app,
                "paste",
                "Paste",
                false,
                Some("Ctrl+V"),
            ).unwrap();

        let insert_uppercase_letter = SubmenuBuilder::new(self.app, "Insert Uppercase Letter")
            .items(&uppercase_letters_list)
            .build().unwrap();
        let insert_lowercase_letter = SubmenuBuilder::new(self.app, "Insert Lowercase Letter")
            .items(&lowercase_letters_list)
            .build().unwrap();
        let insert_numbers = SubmenuBuilder::new(self.app, "Insert Number")
            .items(&numbers_list)
            .build().unwrap();
        let insert_chars = SubmenuBuilder::new(self.app, "Insert Special Character")
            .items(&chars_list)
            .build().unwrap();
        let insert_ascii = SubmenuBuilder::new(self.app, "Insert Character")
            .items(&all_list)
            .build().unwrap();
        let insert_backspace = MenuItem::with_id(
                self.app,
                "backspace",
                "Backspace",
                true,
                Some(""),
            ).unwrap();
        let move_cursor_left = MenuItem::with_id(
                self.app,
                "cursor_left",
                "Move cursor left",
                true,
                Some(""),
            ).unwrap();
        let move_cursor_right = MenuItem::with_id(
                self.app,
                "cursor_right",
                "Move cursor right",
                true,
                Some(""),
            ).unwrap();
        let new_line = MenuItem::with_id(
                self.app,
                "new_line",
                "Enter",
                true,
                Some(""),
            ).unwrap();
        SubmenuBuilder::with_id(self.app, "edit", "Edit")
            .undo()
            .separator()
            .item(&insert_uppercase_letter)
            .item(&insert_lowercase_letter)
            .item(&insert_numbers)
            .item(&insert_chars)
            //.item(&insert_ascii)
            .item(&insert_backspace)
            .item(&new_line)
            .item(&move_cursor_left)
            .item(&move_cursor_right)
            .separator()
            .cut()
            .copy()
            .item(&paste)
            .item(&item)
            .build().unwrap()
    }
    pub fn format_menu(&self) -> tauri::menu::Submenu<Wry> {
        let store = self.app.state::<Arc<Store<Wry>>>();
        let word_wrap = CheckMenuItem::with_id(
                self.app,
                "word_wrap",
                "Word Wrap",
                true,
                store.get("word_wrap").unwrap().as_bool().unwrap(),
                Some(""),
            ).unwrap();
        let font = MenuItem::with_id(
                self.app,
                "font",
                "Font...",
                false,
                Some(""),
            ).unwrap();
        SubmenuBuilder::with_id(self.app, "format", "Format")
            .item(&word_wrap)
            .item(&font)
            .build().unwrap()
    }
    pub fn view_menu(&self) -> tauri::menu::Submenu<Wry> {
        let zoom_in = MenuItem::with_id(
                self.app,
                "zoom_in",
                "Zoom In",
                false,
                Some("Ctrl+Plus"),
            ).unwrap();
        let zoom_out = MenuItem::with_id(
                self.app,
                "zoom_out",
                "Zoom out",
                false,
                Some("Ctrl+Minus"),
            ).unwrap();
        let zoom_default = MenuItem::with_id(
                self.app,
                "zoom_default",
                "Restore Default Zoom",
                false,
                Some("Ctrl+0"),
            ).unwrap();
        let zoom = SubmenuBuilder::new(self.app, "Zoom")
            .item(&zoom_in)
            .item(&zoom_out)
            .item(&zoom_default)
            .build().unwrap();

        let store = self.app.state::<Arc<Store<Wry>>>();
        let status_bar = CheckMenuItem::with_id(
                self.app,
                "status_bar",
                "Status Bar",
                true,
                store.get("show_statusbar").unwrap().as_bool().unwrap(),
                Some(""),
            ).unwrap();

        SubmenuBuilder::with_id(self.app, "view", "View")
            .item(&zoom)
            .item(&status_bar)
            .build().unwrap()
    }
    pub fn help_menu(&self) -> tauri::menu::Submenu<Wry> {
        let view_help = MenuItem::with_id(
                self.app,
                "view_help",
                "View Help",
                false,
                Some(""),
            ).unwrap();
        let send_feedback = MenuItem::with_id(
                self.app,
                "send_feedback",
                "Send Feedback",
                false,
                Some(""),
            ).unwrap();
        let metadata = AboutMetadataBuilder::new()
            .name(Some(self.app.package_info().name.clone()))
            .version(self.app.config().version.clone())
            .license(Some("MIT"))
            .website(Some("www.notepadminusminus.mellobacon.dev/"))
            .credits(Some("mellobacon"))
            .comments(Some("Notepad-- is a notepad meant to be worse than Windows Notepad. Welcome :)"))
            .build();
        SubmenuBuilder::with_id(self.app, "help", "Help")
            .item(&view_help)
            .item(&send_feedback)
            .separator()
            .about(Some(metadata))
            .build().unwrap()
    }
}
