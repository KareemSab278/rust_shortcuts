/*
    disclaimer: only works on mac for lazy automation
    this was created to help me work faster on my mac (dont want to hit command + space then type terminal all the time...)
*/
use rdev::{Event, EventType, Key, listen};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::{path::PathBuf, process::Command};

pub const FILE: &str = "shortcuts.json";

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

            "t" => keys.contains(&Key::KeyT),
            "x" => keys.contains(&Key::KeyX),
            "a" => keys.contains(&Key::KeyA),
            "s" => keys.contains(&Key::KeyS),

            _ => false,
        })
}

fn launch_program(program: &str) {
    let result = Command::new("open").args(["-a", program]).spawn();

    if let Err(error) = result {
        eprintln!("Failed to launch {}: {}", program, error);
    }
}

#[allow(dead_code)]
pub fn list_options() {
    let locations = [
        "/Applications",
        "/System/Applications",
        "/System/Library/CoreServices",
        "/System/Applications/Utilities",
    ];

    println!("=============== Programs & Services ================");
    for location in locations {
        let output = Command::new("ls")
            .arg(location)
            .output()
            .expect("Failed to list applications");

        let apps = String::from_utf8_lossy(&output.stdout);

        println!("\n=== {} ===", location);

        for app in apps.lines() {
            println!("{}", app);
        }
    }
    println!("====================================================");

    println!(
        "
    \n
    \n INSTRUCTIONS:
    \n from root, type micro shortcuts.json to edit the shortcuts file.
    \n add the desired shortcuts in valid JSON.
    \n save the file and the changes will take effect immediately.
    \n\n Example:
    \n
    \n     'shortcut': ['command', 'option', 'X'],
    \n     'program': 'Any program listen above. (like Terminal.app, Notes.app, etc.)'
    \n====================================================
    "
    );
}

pub fn initialize() {
    let shortcuts_path = std::env::home_dir().unwrap().join(FILE);

    if !shortcuts_path.exists() {
        std::fs::write(&shortcuts_path, "[]\n").expect("Failed to create shortcuts file");
    }

    // only set to true if in dev mode - if production it dumps prints to console which is a memory concern
    // list_options();
    // list_shortcuts(shortcuts_path);
    key_listener();
}

#[allow(dead_code)]
pub fn list_shortcuts(path: PathBuf) {
    let contents = std::fs::read_to_string(path).expect("Failed to read shortcuts file");

    let shortcuts: Vec<ShortcutStruct> =
        serde_json::from_str(&contents).expect("Failed to parse shortcuts file");

    if shortcuts.is_empty() {
        println!("No shortcuts found. Add some in the shortcuts.json file.");
        return;
    }
    println!("================ Current Shortcuts =================");

    for shortcut in shortcuts {
        println!(
            "Shortcut: {:?}, Program: {}",
            shortcut.shortcut, shortcut.program
        );
    }
    println!("====================================================");
}
