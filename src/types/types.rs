use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimension {
    height: u16,
    width: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileDetails {
    pub path: Option<String>,
    pub dimension: Option<Dimension>,
    pub size: Option<u128>,
    pub components: Option<u8>,
    pub json: Option<bool>,
}

impl FileDetails {
    pub fn new() -> Self {
        Self {
            path: None,
            dimension: None,
            size: None,
            components: None,
            json: None,
        }
    }
}

#[derive(Debug)]
pub enum ErrorHandler {
    EmptyArguments,
    InvalidFilePath,
    InvalidFileType,
    ByteReadingError,
    EmptyFileArguments,
    EmptyFileBuffer,
    ReadDirError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathType {
    pub absolute_path: String,
    pub buffer: PathBuf,
}

impl Dimension {
    pub fn new(height: u16, width: u16) -> Self {
        Self { height, width }
    }
}
