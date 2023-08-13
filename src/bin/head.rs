use clap::Parser;
use runix::{open_file, Result};

#[derive(Parser, Debug)]
#[command(name = "head", author = "protium", version = "0.1.0", about = "")]
struct Args {
    #[arg(
        name = "files",
        value_name = "FILES",
        default_value = "-",
        help = "Input file(s)"
    )]
    files: Vec<String>,

    #[arg(
        name = "lines",
        value_name = "LINES",
        short = 'n',
        long = "lines",
        help = "Number of lines",
        default_value_t = 10,
        conflicts_with = "bytes"
    )]
    lines: usize,

    #[arg(
        name = "bytes",
        value_name = "BYTES",
        short = 'c',
        long = "bytes",
        help = "Number of bytes"
    )]
    bytes: Option<usize>,
}

fn run(args: Args) -> Result<()> {
    let print_header = args.files.len() > 1;
    for (i, f) in args.files.iter().enumerate() {
        let content = open_file(f);
        if let Err(e) = content {
            eprintln!("head: cannot open '{f}' for reading: {e}");
            continue;
        }

        if i > 0 {
            println!();
        }

        let mut content = content.unwrap();
        if print_header {
            println!("==> {} <==", if f == "-" { "standard input" } else { f });
        }

        if let Some(count) = args.bytes {
            let mut buf: Vec<u8> = vec![0; count];
            content.read_exact(&mut buf)?;
            println!("{}", String::from_utf8_lossy(&buf));
            continue;
        }

        let mut line = String::new();
        for _ in 0..args.lines {
            let b = content.read_line(&mut line)?;
            if b == 0 {
                break;
            }

            print!("{line}");
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
        eprintln!("{e}");
        std::process::exit(1);
    }
}
