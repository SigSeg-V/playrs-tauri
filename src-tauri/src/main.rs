#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod playback;
use playback::init_handle;
use playback::init_sink;
use playback::PlayState;

mod filepicker;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // init audio streams
    let (_stream, stream_handle) = init_handle();
    let sink = init_sink(&stream_handle);

    tauri::Builder::default()
    .manage(PlayState{sink, stream_handle})
        .invoke_handler(tauri::generate_handler![playback::play_sound, playback::add_to_queue, playback::pause_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
