use clap::Parser;
use regex::Regex;
use runix::Result;
use walkdir::WalkDir;

#[derive(Debug, Copy, Clone)]
enum EntryType {
    File,
    Directory,
    Symlink,
}

impl std::str::FromStr for EntryType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "f" => Ok(EntryType::File),
            "d" => Ok(EntryType::Directory),
            "l" => Ok(EntryType::Symlink),
            _ => Err(format!("Invalid entry type: {}", s)),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(
    name = "find",
    author = "protium",
    version = "0.1.0",
    about = "Search for files in directory hierarchy"
)]
struct Args {
    #[arg(name = "depth", short, long)]
    depth: Option<usize>,

    #[arg(name = "type", short, long, help = "File is of type: [f, d, l]")]
    entry_types: Vec<EntryType>,

    #[arg(name = "paths", default_value = ".")]
    paths: Vec<String>,

    #[arg(
        name = "expression",
        required = true,
        help = "Regex expression",
        last = true
    )]
    expr: Regex,
}

fn run(args: Args) -> Result<()> {
    for path in &args.paths {
        let mut iter = WalkDir::new(path);
        if let Some(depth) = args.depth {
            iter = iter.max_depth(depth);
        }

        for entry in iter {
            match entry {
                Ok(entry) => {
                    if !args.entry_types.is_empty()
                        && !args.entry_types.iter().any(|entry_type| match entry_type {
                            EntryType::File => entry.file_type().is_file(),
                            EntryType::Directory => entry.file_type().is_dir(),
                            EntryType::Symlink => entry.file_type().is_symlink(),
                        })
                    {
                        continue;
                    }

                    if !args.expr.is_match(&entry.file_name().to_string_lossy()) {
                        continue;
                    }

                    println!("{}", entry.path().display());
                }
                Err(e) => {
                    if let Some(io) = e.io_error() {
                        eprintln!("{}: {}", path, io);
                    } else {
                        eprintln!("{}", e);
                    }
                }
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
        eprintln!("{e}");
        std::process::exit(1);
    }
}
