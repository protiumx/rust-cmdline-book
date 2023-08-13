use std::io::BufRead;

use clap::Parser;
use runix::{open_file, Result};

#[derive(Parser, Debug)]
#[clap(
    name = "wc",
    author = "protium",
    version = "0.1.0",
    about = "Counts lines, words and bytes"
)]
struct Args {
    #[arg(name = "files", default_value = "-", help = "Input file(s)")]
    files: Vec<String>,

    #[arg(
        name = "count",
        short,
        long,
        help = "The number of bytes in each input",
        conflicts_with = "chars"
    )]
    bytes: bool,

    #[arg(
        name = "chars",
        short = 'm',
        long = "chars",
        help = "The number of chars in each input"
    )]
    chars: bool,

    #[arg(name = "lines", short, long, help = "The number of line in each input")]
    lines: bool,

    #[arg(
        name = "words",
        short,
        long,
        help = "The number of words in each input"
    )]
    words: bool,
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    bytes: usize,
    chars: usize,
    lines: usize,
    words: usize,
}

fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut ret = FileInfo {
        bytes: 0,
        chars: 0,
        lines: 0,
        words: 0,
    };

    let mut line = String::new();
    loop {
        let b = file.read_line(&mut line)?;
        if b == 0 {
            break;
        }
        ret.lines += 1;
        ret.bytes += b;
        ret.chars += line.chars().count();
        ret.words += line.split_whitespace().count();

        line.clear();
    }

    Ok(ret)
}

fn format_info(flag: bool, value: usize) -> String {
    if flag {
        return format!("{:>5}", value).to_string();
    }

    String::new()
}

fn run(args: Args) -> Result<()> {
    let (mut bytes, mut chars, mut lines, mut words) = (0, 0, 0, 0);
    for f in &args.files {
        let file = open_file(f);
        if let Err(e) = file {
            eprintln!("wc: '{f}': {e}");
            continue;
        }

        let info = count(file.unwrap())?;
        let mut out = String::new();
        out.push_str(&format_info(args.lines, info.lines));
        out.push_str(&format_info(args.words, info.words));
        out.push_str(&format_info(args.bytes, info.bytes));
        out.push_str(&format_info(args.chars, info.chars));

        println!(
            "{} {}",
            out,
            if f == "-" { "".to_string() } else { f.clone() }
        );

        bytes += info.bytes;
        chars += info.chars;
        lines += info.lines;
        words += info.words;
    }

    if args.files.len() > 1 {
        let mut out = String::new();
        out.push_str(&format_info(args.lines, lines));
        out.push_str(&format_info(args.words, words));
        out.push_str(&format_info(args.bytes, bytes));
        out.push_str(&format_info(args.chars, chars));

        println!("{out} total");
    }

    Ok(())
}

fn get_args() -> Result<Args> {
    let mut ret = Args::try_parse()?;
    if [ret.lines, ret.chars, ret.bytes, ret.words]
        .iter()
        .all(|v| !v)
    {
        ret.bytes = true;
        ret.lines = true;
        ret.words = true;
    }
    Ok(ret)
}

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "Join the dark side 🌕.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());

        let expected = FileInfo {
            lines: 1,
            words: 5,
            chars: 23,
            bytes: 26,
        };

        assert_eq!(info.unwrap(), expected);
    }
}
