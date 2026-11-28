use std::{fs, io::Read};

use crate::types::types::{Dimension, ErrorHandler, FileDetails, PathType};

pub fn parse_file_bytes(path: PathType, flags: &[String]) -> Result<FileDetails, ErrorHandler> {
    let file_path = &path.absolute_path;
    let mut file_details = FileDetails::new();

    if let Ok(mut file) = fs::File::open(path.buffer) {
        if flags.is_empty() {
            return Err(ErrorHandler::EmptyFileArguments);
        }

        let mut window = [0u8; 2];

        loop {
            file.read_exact(&mut window)
                .map_err(|_| ErrorHandler::ByteReadingError)?;

            if window == [0xFF, 0xC0] || window == [0xFF, 0xC2] {
                let mut rest = [0u8; 8];

                file.read_exact(&mut rest)
                    .map_err(|_| ErrorHandler::ByteReadingError)?;

                let height = u16::from_be_bytes([rest[3], rest[4]]);
                let width = u16::from_be_bytes([rest[5], rest[6]]);

                let components = rest[7];
                let dimensions = Dimension::new(height, width);

                for flag in flags {
                    match flag.as_str() {
                        "--dimensions" => {
                            file_details.dimension = Some(dimensions);
                        }
                        "--components" => {
                            file_details.components = Some(components);
                        }
                        "--json" => file_details.json = Some(true),
                        "--abspath" => file_details.path = Some(file_path.clone()),
                        "--size" => {
                            let size = file
                                .metadata()
                                .map_err(|_| ErrorHandler::InvalidFileType)?
                                .len();

                            file_details.size = Some(size as u128);
                        }
                        _ => {
                            println!("Irrelevant flag");
                        }
                    }
                }

                break;
            }

            window[0] = window[1];

            file.read_exact(&mut window[1..])
                .map_err(|_| ErrorHandler::EmptyFileBuffer)?;
        }

        return Ok(file_details);
    } else {
        return Err(ErrorHandler::InvalidFilePath);
    }
}
