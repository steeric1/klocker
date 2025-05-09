use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
    str::FromStr,
    sync::OnceLock,
};

use serde::Serialize;

static PROJECTS_FILE: OnceLock<PathBuf> = OnceLock::new();

#[derive(Serialize, Default)]
struct Project {
    name: String,
    total_seconds: u64,
    last_worked: u64,
    num_tasks: u32,
}

#[tauri::command]
fn list_projects() -> Vec<Project> {
    // Unwrap: we expect the user to be running on a supported system
    let path = PROJECTS_FILE.get_or_init(|| {
        let mut path = dirs::config_local_dir().unwrap();
        path.push("klocker");
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        }

        path.push("projects.txt");
        if !path.exists() {
            File::create(&path).unwrap();
        }

        path
    });

    let contents = fs::read_to_string(path).unwrap();
    contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let path = PathBuf::from_str(s).unwrap();
            let mut name = path.file_name().unwrap().to_str().unwrap().to_string();
            let _ = name.split_off(name.rfind('.').unwrap());

            Project {
                name,
                ..Default::default()
            }
        })
        .collect()
}

#[tauri::command]
fn create_project(mut dir: PathBuf, name: String) -> Result<(), &'static str> {
    dir.push(name);
    dir.set_extension("csv");

    match dir.try_exists() {
        Ok(true) => return Err("A file by that name already exists"),
        Err(_) => return Err("An unknown error occurred"),
        _ => (),
    }

    let _ = File::create(&dir).map_err(|_| "Failed to create file")?;

    let path = PROJECTS_FILE.get().unwrap();
    let mut file = OpenOptions::new().append(true).open(path).unwrap();

    let mut buf = vec![b'\n'];

    buf.extend(dir.to_string_lossy().as_bytes().iter());

    file.write_all(&buf).unwrap();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![list_projects, create_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
