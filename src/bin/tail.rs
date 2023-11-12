use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    str::FromStr,
};

use clap::Parser;
use runix::{open_file, Result};

#[derive(Clone, Debug, PartialEq)]
enum TakeSize {
    PlusZero,
    Num(i64),
}

impl FromStr for TakeSize {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == "+0" {
            return Ok(Self::PlusZero);
        }

        let mut n: i64 = s.parse()?;
        if !s.starts_with('+') && n > 0 {
            n = -n;
        }
        Ok(Self::Num(n))
    }
}

impl Display for TakeSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlusZero => write!(f, "+0"),
            Self::Num(n) => write!(f, "{n}"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "tail",
    author = "protium",
    version = "0.1.0",
    about = "display the last part of a file"
)]
struct Args {
    #[arg(
        name = "files",
        value_name = "FILES",
        help = "Input file(s)",
        required = true
    )]
    files: Vec<String>,

    #[arg(
        name = "lines",
        short = 'n',
        long = "lines",
        help = "Number of lines",
        default_value_t = TakeSize::Num(10),
        conflicts_with = "bytes"
    )]
    lines: TakeSize,

    #[arg(name = "bytes", short = 'c', long = "bytes", help = "Number of bytes")]
    bytes: Option<TakeSize>,

    #[arg(
        name = "quiet",
        short = 'q',
        long = "quiet",
        help = "never print headers giving file names",
        default_value_t = false
    )]
    quiet: bool,
}

fn run(args: Args) -> Result<()> {
    let print_header = args.files.len() > 1 && !args.quiet;
    for (i, f) in args.files.iter().enumerate() {
        let content = BufReader::new(File::open(f)?);
        if i > 0 {
            println!();
        }

        if print_header {
            println!("==> {} <==", if f == "-" { "standard input" } else { f });
        }

        let (total_lines, total_bytes) = get_lines_bytes(f)?;
        if let Some(count) = &args.bytes {
            print_bytes(content, count, total_bytes)?;
        } else {
            print_lines(content, &args.lines, total_lines)?;
        }
    }

    Ok(())
}

fn get_lines_bytes(filenpath: &str) -> Result<(i64, i64)> {
    let mut content = open_file(filenpath)?;
    let mut buf = Vec::new();
    let mut lines = 0;
    let mut bytes = 0;
    loop {
        let b = content.read_until(b'\n', &mut buf)?;
        if b == 0 {
            break;
        }
        lines += 1;
        bytes += b as i64;

        buf.clear();
    }

    Ok((lines, bytes))
}

fn get_start_index(take_size: &TakeSize, total: i64) -> Option<i64> {
    match take_size {
        TakeSize::PlusZero => {
            if total > 0 {
                Some(0)
            } else {
                None
            }
        }

        TakeSize::Num(n) => {
            let n = *n;
            if n == 0 || total == 0 || n > total {
                return None;
            }

            let start = if n < 0 { total + n } else { n - 1 };
            Some(if start < 0 { 0 } else { start })
        }
    }
}

fn print_lines<T: BufRead>(mut content: T, count: &TakeSize, total_lines: i64) -> Result<()> {
    if let Some(start) = get_start_index(count, total_lines) {
        let mut buf = Vec::new();
        let mut lines = 0;
        loop {
            let b = content.read_until(b'\n', &mut buf)?;
            if b == 0 {
                break;
            }

            if lines >= start {
                print!("{}", String::from_utf8_lossy(&buf));
            }

            lines += 1;
            buf.clear();
        }
    }

    Ok(())
}

fn print_bytes<T: Read + Seek>(mut content: T, count: &TakeSize, total_bytes: i64) -> Result<()> {
    if let Some(start) = get_start_index(count, total_bytes) {
        content.seek(SeekFrom::Start(start as u64))?;
        let mut buf = Vec::new();
        content.read_to_end(&mut buf)?;
        if !buf.is_empty() {
            print!("{}", String::from_utf8_lossy(&buf));
        }
    }

    Ok(())
}

fn get_args() -> Result<Args> {
    Ok(Args::try_parse()?)
}

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_get_start_index() {
        assert_eq!(get_start_index(&TakeSize::PlusZero, 0), None);
        assert_eq!(get_start_index(&TakeSize::PlusZero, 10), Some(0));
        assert_eq!(get_start_index(&TakeSize::Num(0), 0), None);
        assert_eq!(get_start_index(&TakeSize::Num(-10), 100), Some(90));
        assert_eq!(get_start_index(&TakeSize::Num(10), 100), Some(9));
    }
}
