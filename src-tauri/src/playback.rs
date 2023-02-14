use std::{fs::File, thread::sleep};
use std::io::BufReader;
use std::time::Duration;
use rodio::OutputStreamHandle;
use rodio::{Decoder, OutputStream, Sink, source::{Source, SineWave}};

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
pub fn add_to_queue(state: tauri::State<PlayState>) {
    let file = std::fs::File::open("/Users/charlie/Documents/Rust/playrs/src-tauri/assets/music.mp3").unwrap();
    state.sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap()); 
}

#[tauri::command]
pub fn play_sound(state: tauri::State<PlayState>) {
    state.sink.play();
}

#[tauri::command]
pub fn pause_sound(state: tauri::State<PlayState>) {
    state.sink.pause();
}