use clap::Parser;
use runix::{open_file, Result};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::BufRead;

#[derive(Parser, Debug)]
#[command(
    name = "comm",
    author = "protium",
    version = "0.1.0",
    about = "comm compares two sorted files line by line."
)]
struct Args {
    #[arg(value_name = "FILE 1", required = true)]
    file_1: String,
    #[arg(value_name = "FILE 2", required = true)]
    file_2: String,

    #[arg(
        name = "ignore-case",
        short = 'i',
        long = "ignore-case",
        help = "Ignore case distinctions"
    )]
    ignore_case: bool,

    #[arg(
        short = '1',
        help = "suppress column 1 (lines unique to FILE1)",
        default_value_t = false
    )]
    col_1: bool,

    #[arg(
        short = '2',
        help = "suppress column 2 (lines unique to FILE1)",
        default_value_t = false
    )]
    col_2: bool,

    #[arg(
        short = '3',
        help = "suppress column 3 (lines unique to FILE1)",
        default_value_t = false
    )]
    col_3: bool,

    #[arg(
        short = 'd',
        long = "delimiter",
        help = "output delimiter",
        default_value = "\t"
    )]
    delimiter: String,
}

fn run(args: Args) -> Result<()> {
    let file1 = &args.file_1;
    let file2 = &args.file_2;
    if file1 == "-" && file2 == "-" {
        return Err("both input files cannot be STDIN".into());
    }

    let case = |s: String| {
        if args.ignore_case {
            s.to_lowercase()
        } else {
            s
        }
    };

    let mut lines1 = open_file(file1)?
        .lines()
        .map_while(std::result::Result::ok)
        .map(case);
    let mut lines2 = open_file(file2)?
        .lines()
        .map_while(std::result::Result::ok)
        .map(case);

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(a), Some(b)) => match a.cmp(b) {
                Less => {
                    if !args.col_1 {
                        println!("{a}");
                    }
                    line1 = lines1.next();
                }
                Greater => {
                    if !args.col_2 {
                        println!("{}{}", args.delimiter, b);
                    }
                    line2 = lines2.next();
                }
                Equal => {
                    if !args.col_3 {
                        println!("{}{}{}", args.delimiter, args.delimiter, a);
                    }
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
            },

            (Some(line), None) => {
                if !args.col_1 {
                    println!("{line}",);
                }
                line1 = lines1.next();
            }

            (None, Some(line)) => {
                if !args.col_2 {
                    println!("{}{}", args.delimiter, line);
                }
                line2 = lines2.next();
            }

            _ => (),
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
