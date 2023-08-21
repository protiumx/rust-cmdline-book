use std::{
    fs::File,
    io::{BufRead, Write},
};

use clap::Parser;
use runix::{open_file, Result};

#[derive(Parser, Debug)]
#[clap(
    name = "uniq",
    author = "protium",
    version = "0.1.0",
    about = "Filter adjacent matching lines from INPUT (or standard input), writing to OUTPUT (or standard output)."
)]
struct Args {
    #[arg(name = "input", default_value = "-", help = "Input file")]
    input: String,

    #[arg(name = "output", help = "Output file")]
    output: Option<String>,

    #[arg(
        name = "count",
        short,
        long,
        help = "Prefix lines by number of occurrences"
    )]
    count: bool,

    #[arg(
        name = "repeated",
        short = 'd',
        long,
        help = "Only print duplicated lines"
    )]
    repeated: bool,

    #[arg(name = "unique", short, long, help = "Only print unique lines")]
    unique: bool,
}

fn print_count(out: &mut Box<dyn Write>, count: usize, line: &str) -> Result<()> {
    if count > 0 {
        write!(out, "{count:7} {line}")?;
    }

    Ok(())
}

fn print_repeated(out: &mut Box<dyn Write>, count: usize, line: &str) -> Result<()> {
    if count > 1 {
        write!(out, "{line}")?;
    }
    Ok(())
}

fn print_unique(out: &mut Box<dyn Write>, count: usize, line: &str) -> Result<()> {
    if count == 1 {
        write!(out, "{line}")?;
    }
    Ok(())
}

fn print_default(out: &mut Box<dyn Write>, count: usize, line: &str) -> Result<()> {
    if count > 0 {
        write!(out, "{line}")?;
    }

    Ok(())
}

fn run(args: Args) -> Result<()> {
    let mut file = open_file(&args.input)?;
    let mut count: usize = 0;
    let mut previous = String::new();
    let mut line = String::new();
    let mut out: Box<dyn Write> = match &args.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(std::io::stdout()),
    };

    let print = match (args.count, args.repeated, args.unique) {
        (true, _, _) => print_count,
        (_, true, _) => print_repeated,
        (_, _, true) => print_unique,
        _ => print_default,
    };

    loop {
        let b = file.read_line(&mut line)?;
        if b == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(&mut out, count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    print(&mut out, count, &previous)?;
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
