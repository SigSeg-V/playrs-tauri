// Payloads stuff
mod payload;
use payload::OpenFileDialog;

// Tauri stuff
use tauri::{api::dialog::FileDialogBuilder, Window};

#[tauri::command]
pub fn open_file_dialog(window: Window, is_dir: bool) {
    if is_dir {
        open_file_dialog_folders(window);
    }
    else {
        open_file_dialog_files(window);
    }
}

fn open_file_dialog_folders(window: Window) {
    FileDialogBuilder::default()
        .add_filter("Audio Files", &["mp3", "wav", "ogg"])
        .pick_folders(move |path_buf| {

            // Emit that folders have been picked to add to queue
            if let Some(p) = path_buf {
                window.emit("open-folders", OpenFileDialog{paths: p})
                .expect("Error opening folders");
            }

        });
}

fn open_file_dialog_files(window: Window) {
    FileDialogBuilder::default()
        .add_filter("Audio Files", &["mp3", "wav", "ogg"])
        .pick_files(move |path_buf| {

            // Emit that files have been picked and need to be added to queue
            if let Some(p) = path_buf {
                window.emit("open-files", OpenFileDialog{paths: p})
                .expect("Error opening files");
            }

        });
}