use std::{path::PathBuf};

use tauri::{api::dialog::FileDialogBuilder, Window};


#[derive(Clone, serde::Serialize)]
pub struct Payload {
    files: Vec<PathBuf>
}

#[tauri::command]
pub fn open_file_dialog(window: Window) {
    
    FileDialogBuilder::default()
          //.add_filter("Markdown", &["md"])
          .pick_files(move |path_buf| {

            let mut files: Vec<PathBuf> = vec![];

            match path_buf {
            Some(p) => { 
                files = p;
             }
            _ => {  }
          };

          window.emit("open-files", Payload{files})
            .expect("Error opening files");

        });
}