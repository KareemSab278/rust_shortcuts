/*
    disclaimer: only works on mac for lazy automation
    this was created to help me work faster on my mac (dont want to hit command + space then type terminal all the time...)
*/

use crate::k_list;
use rdev::{Event, EventType, Key, listen};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::process::Command;

const FILE: &str = "shortcuts.json";

#[derive(Debug, Deserialize)]
pub struct ShortcutStruct {
    pub shortcut: Vec<String>,
    pub program: String,
}


pub fn key_listener() {
    let shortcuts_path = std::env::home_dir().unwrap().join(FILE);

    let contents = std::fs::read_to_string(&shortcuts_path).expect("Failed to read shortcuts file");

    let shortcuts: Vec<ShortcutStruct> =
        serde_json::from_str(&contents).expect("Failed to parse shortcuts file");

    let pressed_keys = Arc::new(Mutex::new(Vec::<Key>::new()));

    let keys = pressed_keys.clone();

    if let Err(error) = listen(move |event| {
        callback(event, &shortcuts, &keys);
    }) {
        eprintln!("Keyboard listener error: {:?}", error);
    }
}


fn callback(event: Event, shortcuts: &[ShortcutStruct], pressed_keys: &Arc<Mutex<Vec<Key>>>) {
    match event.event_type {
        EventType::KeyPress(key) => {
            let mut keys = pressed_keys.lock().unwrap();

            if !keys.contains(&key) {
                keys.push(key);
            }

            for shortcut in shortcuts {
                if shortcut_matches(&keys, &shortcut.shortcut) {
                    launch_program(&shortcut.program);

                    keys.clear();
                    break;
                }
            }
        }

        EventType::KeyRelease(key) => {
            let mut keys = pressed_keys.lock().unwrap();
            keys.retain(|k| *k != key);
        }

        _ => {}
    }
}


fn shortcut_matches(keys: &[Key], shortcut: &[String]) -> bool {
    shortcut
        .iter()
        .all(|key| match key.to_lowercase().as_str() {
            "command" => keys.contains(&Key::MetaLeft) || keys.contains(&Key::MetaRight),

            "option" | "alt" => keys.contains(&Key::Alt),

            "control" | "ctrl" => {
                keys.contains(&Key::ControlLeft) || keys.contains(&Key::ControlRight)
            }

            "shift" => keys.contains(&Key::ShiftLeft) || keys.contains(&Key::ShiftRight),

            _ => k_list::all()
                .get(key.as_str())
                .is_some_and(|key| keys.contains(key)),

            _ => false,
        })
}


fn launch_program(program: &str) {
    let result = Command::new("open").args(["-a", program]).spawn();

    if let Err(error) = result {
        eprintln!("Failed to launch {}: {}", program, error);
    }
}

pub fn initialize() {
    let shortcuts_path = std::env::home_dir().unwrap().join(FILE);

    if !shortcuts_path.exists() {
        std::fs::write(&shortcuts_path, "[]\n").expect("Failed to create shortcuts file");
    }
    key_listener();
}
