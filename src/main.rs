mod helpers;
mod parse;
mod types;

use crate::helpers::path_resolver::read_directory;
use crate::types::types::ErrorHandler;
use parse::parse_file::parse_file_bytes;
use std::path::Path;
use std::{env, thread};

fn main() -> Result<(), ErrorHandler> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let [path_flag, path_val] = &arguments[0..=1] else {
        return Err(ErrorHandler::EmptyArguments);
    };

    if path_flag != "--path" {
        return Err(ErrorHandler::EmptyArguments);
    }

    let paths = read_directory(Path::new(path_val));
    let mut handles = vec![];

    match paths {
        Ok(paths) => {
            for path in paths {
                let flags = arguments.clone();
                let h = thread::spawn(move || parse_file_bytes(path, &flags[2..]));

                handles.push(h);
            }
        }
        Err(_) => return Err(ErrorHandler::ReadDirError),
    }

    for handle in handles {
        let result = handle.join().unwrap();

        if let Ok(res) = result {
            println!("{:#?}", res);
            println!();
        }
    }

    Ok(())
}
