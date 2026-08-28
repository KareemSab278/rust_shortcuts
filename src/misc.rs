use crate::key_listeners::ShortcutStruct;
use std::path::PathBuf;
use std::process::Command;

/*
    not really using this code tbh... just left it here if you want it :)
    dont recommend keeping it in build
*/

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
