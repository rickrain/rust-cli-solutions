use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Ok(f) => {
                //println!("Opened {}", filename);
                let lines_iter = f.lines();
                let mut line_number = 1;

                for txt in lines_iter {
                    let line = txt?;
                    if config.number_lines
                        || (config.number_nonblank_lines && !line.trim().is_empty())
                    {
                        println!("{:>6}\t{}", line_number, line);
                        line_number += 1;
                    } else {
                        println!("{}", line);
                    }
                }
            }
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Rick")
        .about("Rust cat")
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("number nonempty output lines")
                .takes_value(false)
                .conflicts_with("number_lines"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("number all output lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
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

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
