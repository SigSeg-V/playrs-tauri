
use std::io::BufReader;
use std::sync::Mutex;

use gstreamer as gst;
use gstreamer_player as gst_player;
use gst::prelude::*;

use gst_player::{PlayerGMainContextSignalDispatcher, Player, PlayerSignalDispatcher, PlayerVideoRenderer};

pub struct PlayState {
    pub sink: Mutex<Sink>,
}
pub struct Sink {
    pub player: Player,
    pub playlist: Vec<String>,
    pub current_file: usize,
}

#[tauri::command]
pub fn add_to_queue(state: tauri::State<PlayState>, files: Vec<String>) {

    let mut sink = state.sink.lock().unwrap();

    // set index for the current file to the first song if not already set
    if sink.playlist.is_empty() {
        sink.current_file = 0;
    }

    // default music file for testing
    if files.is_empty() {
        let file = "file:///Users/charlie/Documents/Rust/playrs/src-tauri/assets/Scarlet Fire.mp3".to_string();
        sink.playlist.push(file);
    }

    // coming from the file browser
    else {
        for file in files {
            sink.playlist.push("file://".to_string() + &file);
        }
    }
}

#[tauri::command]
pub fn load_file(state: tauri::State<PlayState>) {
    let sink = state.sink.lock().unwrap();
    sink.player.set_uri(Some(sink.playlist[sink.current_file].as_str()));
}

#[tauri::command]
pub fn play_sound(state: tauri::State<PlayState>) {
    let sink = state.sink.lock().unwrap();
    if sink.player.uri().is_none() {
        sink.player.set_uri(Some(sink.playlist[sink.current_file].as_str()));
    }
    sink.player.play();
}

#[tauri::command]
pub fn pause_sound(state: tauri::State<PlayState>) {
    let sink = state.sink.lock().unwrap();
    sink.player.pause();
}

#[tauri::command]
pub fn stop_sound(state: tauri::State<PlayState>) {
    let sink = state.sink.lock().unwrap();
    sink.player.stop();
}