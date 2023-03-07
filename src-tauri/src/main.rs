#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use gst::glib;
use gst::glib::translate::FromGlibPtrFull;
use gstreamer as gst;
use gstreamer_player as gst_player;

use gstreamer::prelude::*;
use gstreamer_player::prelude::*;

mod filepicker;
mod playback;

use playback::PlayState;
use playback::Sink;

use std::sync::Mutex;

fn main() -> Result<(), glib::Error> {
    // init audio streams
    gst::init()?;
    let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
    let player = gst_player::Player::new(
        None::<gst_player::PlayerVideoRenderer>,
        Some(dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()),
    );

    tauri::Builder::default()
        .manage(
                PlayState{
                    sink: Mutex::new(
                            Sink{player,
                                 playlist: vec!["file:///Users/charlie/Documents/Rust/playrs/src-tauri/assets/Scarlet Fire.mp3".to_string()],
                                 current_file: 0
                            })
                })
        .invoke_handler(tauri::generate_handler![
            playback::play_sound, 
            playback::pause_sound,
            playback::stop_sound,
            playback::add_to_queue, 
            filepicker::open_file_dialog,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
