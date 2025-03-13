use std::path::{Path, PathBuf};

pub fn join(base_path: &str, path: &str) -> String {
    let full_path: &Path = Path::new(base_path);
    let full_path: PathBuf = full_path.join(path);
    full_path.to_string_lossy().into_owned()
}
