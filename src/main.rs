use std::env;
use std::fs;
use std::io::Read;

#[derive(Debug)]
enum ErrorHandler {
    EmptyFilePath,
    EmptyArguments,
    InvalidFilePath,
    InvalidFileType,
    InvalidFileHeader,
}

fn parse_file_bytes1(path_val: &String) -> Result<(), ErrorHandler> {
    let file_path = path_val.as_str();

    if let Ok(mut file) = fs::File::open(file_path) {
        let mut jpg_magic = [0u8; 2];

        match file.read_exact(&mut jpg_magic) {
            Ok(_) => {
                if jpg_magic == [0xFF, 0xD8] {
                    println!("This is a JPG file");
                    return Ok(());
                } else {
                    return Err(ErrorHandler::InvalidFileType);
                }
            }
            _ => return Err(ErrorHandler::InvalidFileHeader),
        }
    } else {
        return Err(ErrorHandler::InvalidFilePath);
    }
}

fn main() -> Result<(), ErrorHandler> {
    let arguments: Vec<String> = env::args().enumerate().map(|(_, f)| f).collect();
    let [path_flag, path_val] = &arguments[1..=2] else {
        return Err(ErrorHandler::EmptyArguments);
    };

    if path_flag != "--path" {
        return Err(ErrorHandler::EmptyArguments);
    }

    if path_val.is_empty() {
        return Err(ErrorHandler::EmptyFilePath);
    }

    let parsed_content = parse_file_bytes1(&path_val);

    parsed_content
}
