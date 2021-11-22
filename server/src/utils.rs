use std::path::PathBuf;

pub fn get_data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("You're running an unsupported operating system")
        .join("dnd-stuff")
}
