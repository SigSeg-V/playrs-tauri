use std::path::PathBuf;

#[derive(Clone, serde::Serialize)]
pub struct OpenFileDialog {
    pub paths: Vec<PathBuf>
}
