use clap::Parser;
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

fn open(filename: &str) -> runix::Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn get_args() -> runix::Result<Args> {
    Ok(Args::parse())
}

fn run(conf: Args) -> runix::Result<()> {
    for filename in conf.files {
        match open(&filename) {
            Ok(content) => {
                let mut n = 0;
                for line in content.lines() {
                    let line = line?;
                    if conf.number_lines {
                        println!("{:6}\t{}", n + 1, line);
                        n += 1;
                    } else if conf.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            println!("{:6}\t{}", n + 1, line);
                            n += 1;
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
            Err(e) => eprintln!("{} err", e),
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    std::process::exit(0);
}
