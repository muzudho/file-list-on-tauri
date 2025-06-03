use std::fs::{self};
use std::path::Path;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// ディレクトリー内のファイル名を取得するTauriコマンド。
// 読取成功時はファイル名のリストを、失敗時はエラーメッセージを返す。
#[tauri::command]
fn get_file_names(dir_path: String) -> Result<Vec<String>, String> {
    let path = Path::new(&dir_path);
    if !path.is_dir() { // ディレクトリーを指定していなければエラー。
        return Err(format!("{} is not a valid directory", dir_path));
    }

    let mut file_names = Vec::new();
    match fs::read_dir(path) {
        Ok(entries) => {    // 読取成功時。
            for entry in entries {  // ファイルまたはディレクトリーを１つずつ。
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() { // ファイル名を取出し。
                        file_names.push(file_name.to_string());
                    }
                }
            }
            Ok(file_names)
        }
        Err(e) => Err(format!("Failed to read directory: {}", e)), // 読取失敗時。
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_file_names
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
