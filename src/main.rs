mod helpers;
mod parse;
mod types;

use crate::helpers::path_resolver::path_resolver;
use crate::types::types::ErrorHandler;
use parse::parse_file::parse_file_bytes;
use std::env;

fn main() -> Result<(), ErrorHandler> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let [path_flag, path_val] = &arguments[0..=1] else {
        return Err(ErrorHandler::EmptyArguments);
    };

    if path_flag != "--path" {
        return Err(ErrorHandler::EmptyArguments);
    }

    let path = path_resolver(path_val)?.to_owned();

    let extra_flags: Vec<&str> = arguments[2..].iter().map(|f| f.as_str()).collect();

    let parsed_content = parse_file_bytes(path, extra_flags);

    println!("{:#?}", parsed_content.ok());

    Ok(())
}
