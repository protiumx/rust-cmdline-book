use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Opens a file and returns a BufRead
/// Treats '-' as stdin
pub fn open_file(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => {
            let file = File::open(filename);
            match file {
                Ok(file) => Ok(Box::new(BufReader::new(file))),
                Err(e) => Err(format!("{filename}: {e}").into()),
            }
        }
    }
}
