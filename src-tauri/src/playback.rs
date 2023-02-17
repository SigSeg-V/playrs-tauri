
use std::io::BufReader;

use rodio::OutputStreamHandle;
use rodio::{OutputStream, Sink};

pub struct PlayState {
    pub sink: Sink,
    pub stream_handle: OutputStreamHandle
}

pub fn init_handle() -> (OutputStream, OutputStreamHandle) {
    OutputStream::try_default().unwrap()
}

pub fn init_sink(stream_handle: &OutputStreamHandle) -> Sink {
    Sink::try_new(stream_handle).unwrap()
}

#[tauri::command]
pub fn add_to_queue(state: tauri::State<PlayState>, files: Vec<String>) -> i32 {

    let mut num_files: i32 = 0;

    // default music file for testing
    if files.is_empty() {
        let file = std::fs::File::open("/Users/charlie/Documents/Rust/playrs/src-tauri/assets/Scarlet Fire.mp3").unwrap();
        state.sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap()); 
        num_files = 1;
    }
    // coming from the file browser
    else {
        num_files = files.len() as i32;
        for file_path in files {
            let file = std::fs::File::open(file_path).unwrap();
            state.sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());  
        }
    }
    num_files
}

#[tauri::command]
pub fn play_sound(state: tauri::State<PlayState>) {
    state.sink.play();
}

#[tauri::command]
pub fn pause_sound(state: tauri::State<PlayState>) {
    state.sink.pause();
}

#[tauri::command]
pub fn stop_sound(state: tauri::State<PlayState>) {
   state.sink.stop(); 
}

#[tauri::command]
pub fn get_sound_runtime(state: tauri::State<PlayState>) {
  
  todo!()

}