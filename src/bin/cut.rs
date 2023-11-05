use clap::Parser;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use runix::{open_file, Result};
use std::ops::Range;

type Ranges = Vec<Range<usize>>;

#[derive(Debug)]
enum Output {
    Bytes,
    Chars,
    Fields,
}

#[derive(Parser, Debug)]
#[command(
    name = "cut",
    author = "protium",
    version = "0.1.0",
    about = "Cut bytes, charachters, or fields from files. Lists are zero indexed"
)]
struct Args {
    #[arg(
        name = "files",
        value_name = "FILES",
        default_value = "-",
        help = "Input file(s)"
    )]
    files: Vec<String>,

    #[arg(
        name = "bytes",
        value_name = "BYTES",
        short = 'b',
        long = "bytes",
        help = "Select only these bytes. Zero based",
        conflicts_with_all = &["charachters", "fields"],
        value_parser = parse_ranges,
    )]
    bytes: Option<Ranges>,

    // E.g. -c 1,3,5-7.
    #[arg(
        name = "charachters",
        value_name = "CHARS",
        short = 'c',
        long = "charachters",
        help = "Select only these charachters. Zero based",
        conflicts_with_all = &["bytes", "fields"],
        value_parser = parse_ranges,
    )]
    chars: Option<Ranges>,

    #[arg(
        name = "fields",
        value_name = "FIELDS",
        short = 'f',
        long = "fields",
        help = "Select only these fields. Zero based",
        conflicts_with_all = &["bytes", "charachters"],
        value_parser = parse_ranges,
    )]
    fields: Option<Ranges>,

    // NOTE: should respect escaped delimiters
    #[arg(
        name = "delimiters",
        value_name = "DELIM",
        short = 'd',
        long = "delimiter",
        default_value = "\t",
        value_parser = parse_delimiter,
        help = "Use DELIM instead of TAB for field delimiter"
    )]
    delim: u8,
}

fn parse_delimiter(delim: &str) -> std::result::Result<u8, String> {
    let delim_bytes = delim.as_bytes();
    if delim_bytes.len() != 1 {
        return Err("--delim must be a single byte".into());
    }

    Ok(delim_bytes[0])
}

fn parse_ranges(range: &str) -> std::result::Result<Ranges, String> {
    let range_expr = regex::Regex::new(r"^(\d+)(-\d+)?$").unwrap();
    range
        .split(',')
        .map(|r| {
            range_expr
                .captures(r)
                .ok_or_else(|| format!("invalid list value: {:?}", r))
                .and_then(|captures| {
                    let first = captures[1].to_string();
                    let first = first
                        .parse::<usize>()
                        .map_err(|_| format!("invalid value: \"{}\"", first))?;

                    if captures.get(2).is_none() {
                        return Ok(first..first + 1);
                    }

                    let second = captures[2].to_string();
                    let second = second[1..] // remove leading dash
                        .parse::<usize>()
                        .map_err(|_| format!("invalid value: \"{}\"", second))?;

                    if first >= second {
                        return Err("first number in range must be lower than second".into());
                    }

                    Ok(first..second)
                })
        })
        .collect::<std::result::Result<_, _>>()
        .map_err(From::from)
}

fn extract_chars(line: &str, chars: &[Range<usize>]) -> String {
    line.chars()
        .collect::<Vec<_>>()
        .iter()
        .enumerate()
        .flat_map(|(i, c)| {
            if chars.iter().any(|r| r.contains(&i)) {
                Some(c)
            } else {
                None
            }
        })
        .collect()
}

fn extract_bytes(line: &str, chars: &[Range<usize>]) -> String {
    let b: Vec<_> = line
        .bytes()
        .enumerate()
        .flat_map(|(i, c)| {
            if chars.iter().any(|r| r.contains(&i)) {
                Some(c)
            } else {
                None
            }
        })
        .collect();

    String::from_utf8_lossy(&b).into_owned()
}

fn extract_fields<'a>(record: &'a StringRecord, ranges: &[Range<usize>]) -> Vec<&'a str> {
    record
        .iter()
        .enumerate()
        .filter_map(|(i, field)| {
            if ranges.iter().any(|r| r.contains(&i)) {
                Some(field)
            } else {
                None
            }
        })
        .collect()
}

fn run(args: Args) -> Result<()> {
    let (ranges, output) = if let Some(ranges) = args.bytes {
        (ranges, Output::Bytes)
    } else if let Some(ranges) = args.chars {
        (ranges, Output::Chars)
    } else if let Some(ranges) = args.fields {
        (ranges, Output::Fields)
    } else {
        unreachable!()
    };

    for (i, f) in args.files.iter().enumerate() {
        let content = open_file(f);
        if let Err(e) = content {
            eprintln!("cut: cannot open '{f}' for reading: {e}");
            continue;
        }

        if i > 0 {
            println!();
        }

        let mut content = content.unwrap();
        match output {
            Output::Bytes => {
                let mut line = String::new();
                loop {
                    let bytes = content.read_line(&mut line)?;
                    if bytes == 0 {
                        break;
                    }

                    println!("{}", extract_bytes(&line, &ranges));
                }
            }
            Output::Chars => {
                let mut line = String::new();
                loop {
                    let bytes = content.read_line(&mut line)?;
                    if bytes == 0 {
                        break;
                    }

                    println!("{}", extract_chars(&line, &ranges));
                }
            }
            Output::Fields => {
                let mut reader = ReaderBuilder::new()
                    .delimiter(args.delim)
                    .has_headers(false)
                    .from_reader(content);

                let mut writer = WriterBuilder::new()
                    .delimiter(args.delim)
                    .from_writer(std::io::stdout());

                for result in reader.records() {
                    let record = result?;
                    writer.write_record(extract_fields(&record, &ranges))?;
                }
            }
        }
    }

    Ok(())
}

fn get_args() -> Result<Args> {
    let args = Args::try_parse()?;

    if args.bytes.is_none() && args.chars.is_none() && args.fields.is_none() {
        return Err("must have --fields, --bytes, or --chars".into());
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
    use super::*;

    #[test]
    fn test_parse_ranges() {
        assert!(parse_ranges("").is_err());

        let r = parse_ranges("+1");
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().to_string(), "invalid list value: \"+1\"");

        let r = parse_ranges("1-a");
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().to_string(), "invalid list value: \"1-a\"");

        let r = parse_ranges("1-1");
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            "first number in range must be lower than second"
        );

        assert_eq!(parse_ranges("0").unwrap(), vec![0..1]);
        assert_eq!(parse_ranges("0,2").unwrap(), vec![0..1, 2..3]);
        assert_eq!(parse_ranges("1-3").unwrap(), vec![1..3]);
        assert_eq!(parse_ranges("0,6,2-5").unwrap(), vec![0..1, 6..7, 2..5]);
    }

    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_chars("", &[0..1]), "".to_string());
        assert_eq!(extract_chars("á", &[0..1]), "á".to_string());
        assert_eq!(extract_chars("ábc", &[0..1, 1..2]), "áb".to_string());
        assert_eq!(extract_chars("ábc", &[0..3]), "ábc".to_string());
        assert_eq!(extract_chars("ábc", &[2..3, 1..2]), "bc".to_string());
        assert_eq!(extract_chars("ábc", &[0..1, 1..2, 4..5]), "áb".to_string());
    }

    #[test]
    fn test_extract_bytes() {
        assert_eq!(extract_bytes("á", &[0..1]), "�".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2]), "á".to_string());
        assert_eq!(extract_bytes("ábc", &[0..3]), "áb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..4]), "ábc".to_string());
        assert_eq!(extract_bytes("ábc", &[3..4, 2..3]), "bc".to_string());
    }

    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Name", "Description", "Number"]);
        assert_eq!(extract_fields(&rec, &[0..1]), vec!["Name"]);
        assert_eq!(extract_fields(&rec, &[1..2]), vec!["Description"]);
        assert_eq!(
            extract_fields(&rec, &[1..2, 0..1]),
            vec!["Name", "Description"]
        );
        assert_eq!(extract_fields(&rec, &[2..3, 0..1]), vec!["Name", "Number"]);
    }
}
