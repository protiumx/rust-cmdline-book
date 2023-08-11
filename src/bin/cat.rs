use clap::Parser;
use runix::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[clap(
    name = "cat",
    author = "protium",
    version = "0.1.0",
    about = "Concatenates files"
)]
struct Args {
    #[clap(name = "files", default_value = "-", help = "Input file(s)")]
    files: Vec<String>,

    #[clap(
        name = "number_lines",
        short,
        long = "number",
        help = "Number lines",
        conflicts_with = "number_nonblank_lines"
    )]
    number_lines: bool,

    #[clap(
        name = "number_nonblank_lines",
        short = 'b',
        long = "number-nonblank",
        help = "Number non-blank lines"
    )]
    number_nonblank_lines: bool,
}

fn run(args: Args) -> Result<()> {
    for content in args.files.iter().map(|f| runix::open_file(f)) {
        if let Err(e) = content {
            eprintln!("{e}");
            continue;
        }

        let content = content.unwrap();
        let mut n = 0;
        for line in content.lines() {
            let line = line?;
            if args.number_lines {
                println!("{:6}\t{}", n + 1, line);
                n += 1;
            } else if args.number_nonblank_lines {
                if line.is_empty() {
                    println!();
                } else {
                    println!("{:6}\t{}", n + 1, line);
                    n += 1;
                }
            } else {
                println!("{line}");
            }
        }
    }

    Ok(())
}

fn get_args() -> Result<Args> {
    Ok(Args::try_parse()?)
}

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
