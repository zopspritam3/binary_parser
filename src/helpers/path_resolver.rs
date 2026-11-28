use crate::types::types::{ErrorHandler, PathType};
use std::fs;
use std::path::Path;

pub fn read_directory(path_value: &Path) -> Result<Vec<PathType>, ErrorHandler> {
    let mut parsed_content = fs::read_dir(path_value)
        .map_err(|_| ErrorHandler::ReadDirError)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path());

    let mut result: Vec<PathType> = Vec::new();

    while let Some(path) = parsed_content.next() {
        if path.is_dir() {
            let mut new_temp = read_directory(&path)?;
            result.append(&mut new_temp);
        } else if path.is_file() {
            result.push(PathType {
                absolute_path: path.to_string_lossy().to_string(),
                buffer: path,
            });
        }
    }

    Ok(result)
}
