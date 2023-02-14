use std::{path::PathBuf, io::BufReader, fs::File};

use tauri::api::dialog::FileDialogBuilder;
use crate::playback::PlayState;

#[tauri::command]
pub fn get_file_path(state: tauri::State<PlayState>) {

    let mut files: Vec<PathBuf> = vec![];
    let builder = FileDialogBuilder::new();
    let files = builder
    .pick_files(move |paths| {
        
        // don't need to add anything if the user closes dialog box without selection
        if paths.is_none() {
            return;
        }

        for path in paths.unwrap() {
            
           files.push(path);
        }
    });
        
    // // now we got the pathbufs, we can add them to state
    // for file_path in files {
    //     let file = File::open(file_path).expect("Error in opening files");
    //     state.sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    // }

    todo!()
}