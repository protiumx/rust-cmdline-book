use clap::Parser;
use runix::{open_file, Result};

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
    for f in args.files {
        let content = open_file(&f);
        if let Err(e) = content {
            eprintln!("{e}");
            continue;
        }

        let mut content = content.unwrap();
        let mut n = 0;
        let mut line = String::new();
        loop {
            let bytes = content.read_line(&mut line)?;
            if bytes == 0 {
                break;
            }

            if args.number_lines {
                print!("{:6}\t{}", n + 1, line);
                n += 1;
            } else if args.number_nonblank_lines {
                if line == "\n" || line == "\r\n" {
                    println!();
                } else {
                    print!("{:6}\t{}", n + 1, line);
                    n += 1;
                }
            } else {
                print!("{line}");
            }
            line.clear();
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
