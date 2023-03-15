#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use gst::glib;
use gstreamer as gst;
use gstreamer_player as gst_player;

use gstreamer::prelude::*;

mod filepicker;
mod playback;

use playback::PlayState;
use playback::Sink;

use tauri::AppHandle;
use tauri::Manager;
use tauri::async_runtime::spawn;

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
                                 playlist: vec![],
                                 current_file: 0
                            })
                })
        .setup(|app| {
            let app_handle = app.handle().clone();
            spawn(event_loop(app_handle));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![

            // Audio sink callbacks
            playback::play_sound, 
            playback::pause_sound,
            playback::stop_sound,
            playback::add_to_queue, 
            playback::load_file,
            playback::pop_playlist,
            playback::get_duration,
            playback::get_position,

            // file dialog callbacks
            filepicker::open_file_dialog,
            filepicker::open_folder_dialog,

            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

async fn event_loop(app_handle: AppHandle) {
    loop {

        // Wait for a signal from the receiver
        println!("getting duration of current song");
        playback::get_duration(
            app_handle
                .get_window("main")
                .expect("No applicable window"),
             app_handle
                .try_state
                ::<PlayState>()
                .expect("No state")
        );

        println!("geting position in current song");
        playback::get_position(
            app_handle
                .get_window("main")
                .expect("No applicable window"),
            app_handle
            .try_state
            ::<PlayState>()
            .expect("No state")
        );
        
        // Wait for some time before checking again
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}