use crate::types::types::{ErrorHandler, PathType};
use std::path::Path;

pub fn path_resolver(path: &str) -> Result<PathType, ErrorHandler> {
    if path.is_empty() {
        return Err(ErrorHandler::EmptyFilePath);
    }

    let path = Path::new(path);
    let abs_path = path.canonicalize();

    match abs_path {
        Ok(val) => {
            let abs = val.to_string_lossy().into_owned();

            Ok(PathType::new(abs, val))
        }
        Err(_) => return Err(ErrorHandler::InvalidFilePath),
    }
}
