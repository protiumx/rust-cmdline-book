use std::{fs, io::BufRead, mem};

use clap::Parser;
use regex::{Regex, RegexBuilder};
use runix::{open_file, Result};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(
    name = "grep",
    author = "protium",
    version = "0.1.0",
    about = "grep searches for a pattern in the input files, selecting lines that match that pattern."
)]
struct Args {
    #[arg(
        name = "pattern",
        value_name = "PATTERN",
        help = "Search pattern",
        required = true
    )]
    pattern: Regex,

    #[arg(
        name = "files",
        value_name = "FILES",
        default_value = "-",
        help = "Input file(s)"
    )]
    files: Vec<String>,

    #[arg(
        name = "count",
        help = "Count occurrences",
        short = 'c',
        long = "count"
    )]
    count: bool,

    #[arg(
        name = "ignore-case",
        short = 'i',
        long = "ignore-case",
        help = "Ignore case distinctions"
    )]
    ignore_case: bool,

    #[arg(
        name = "invert-match",
        help = "Inverse match",
        short = 'v',
        long = "invert-match"
    )]
    invert_match: bool,

    #[arg(
        name = "recursive",
        help = "Recursive search",
        short = 'r',
        long = "recursive"
    )]
    recursive: bool,
}

fn run(args: Args) -> Result<()> {
    let files = find_files(&args.files, args.recursive);
    let print = if files.len() > 1 {
        |fname: &str, line: &str| {
            print!("{}:{}", fname, line);
        }
    } else {
        |_: &str, line: &str| {
            print!("{}", line);
        }
    };

    for f in files {
        match f {
            Ok(filename) => match open_file(&filename) {
                Ok(content) => {
                    let lines = find_lines(content, &args.pattern, args.invert_match);
                    if let Err(e) = lines {
                        eprintln!("{e}");
                        continue;
                    }

                    let lines = lines.unwrap();
                    if args.count {
                        print(&filename, &format!("{}\n", lines.len()));
                    } else {
                        for line in &lines {
                            print(&filename, line);
                        }
                    }
                }
                Err(e) => eprintln!("{e}"),
            },

            Err(e) => eprintln!("{e}"),
        }
    }

    Ok(())
}

fn find_lines<T: BufRead>(
    mut content: T,
    pattern: &Regex,
    inverse_match: bool,
) -> Result<Vec<String>> {
    let mut lines = Vec::new();
    let mut line = String::new();
    loop {
        let bytes = content.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if pattern.is_match(&line) ^ inverse_match {
            lines.push(mem::take(&mut line));
        }

        line.clear();
    }

    Ok(lines)
}

fn find_files(paths: &[String], recursive: bool) -> Vec<Result<String>> {
    let mut files = Vec::new();
    for path in paths {
        if path == "-" {
            files.push(Ok(path.to_string()));
            continue;
        }

        match fs::metadata(path) {
            Ok(meta) => {
                if meta.is_file() {
                    files.push(Ok(path.to_string()));
                    continue;
                }

                if meta.is_dir() {
                    if recursive {
                        for entry in WalkDir::new(path)
                            .into_iter()
                            .flatten()
                            .filter(|e| e.file_type().is_file())
                        {
                            files.push(Ok(entry.path().display().to_string()));
                        }
                    } else {
                        files.push(Err(From::from(format!("{} is a directory", path))));
                    }
                }
            }

            Err(e) => files.push(Err(From::from(format!("{}: {}", path, e)))),
        }
    }

    files
}

fn get_args() -> Result<Args> {
    let mut args = Args::try_parse()?;
    if args.ignore_case {
        args.pattern = RegexBuilder::new(args.pattern.as_str())
            .case_insensitive(true)
            .build()?;
    }

    Ok(args)
}

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::find_files;
    use rand::{distributions::Alphanumeric, Rng};

    #[test]
    fn test_find_files() {
        // Simple file
        let files = find_files(&["./tests/inputs/cat_empty.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/cat_empty.txt");

        // Fail when directory and not recursive
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // Non existent
        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }
}
