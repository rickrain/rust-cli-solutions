use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    let multiple_files = config.files.len() > 1;
    for (file_num, file) in config.files.iter().enumerate() {
        match open(&file) {
            Err(e) => eprintln!("{}: {}", file, e),
            Ok(mut file_buffer) => {
                if multiple_files {
                    println!("==> {} <==", file);
                }
                match config.bytes {
                    Some(b) => {
                        let mut buffer = vec![0; b];
                        let mut handle = file_buffer.take(b as u64);
                        let bytes_read = handle.read(&mut buffer);
                        print!("{}", String::from_utf8_lossy(&buffer[0..bytes_read?]));
                    }
                    None => {
                        let mut line = String::new();
                        for _ in 0..config.lines {
                            file_buffer.read_line(&mut line)?;
                            print!("{}", line);
                            line.clear();
                        }
                    }
                }

                if multiple_files && (file_num < config.files.len() - 1) {
                    println!();
                }
            }
        }
    }

    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Rick")
        .about("Rust head")
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .help("print the first NUM bytes of each file")
                .takes_value(true),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("print the first NUM lines instead of the first 10")
                .default_value("10"),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .allow_invalid_utf8(true)
                .help("Input file(s)")
                .multiple_occurrences(true)
                .default_value("-"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(x) if x > 0 => Ok(x),
        _ => Err(val.into()),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
