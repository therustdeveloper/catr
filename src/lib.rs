use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser,Debug)]
#[command(
    author = "William Munoz",
    version,
    about = "Incomplete `GNU cat` in Rust for learning purposes"
)]
pub struct Config {
    /// files to cat
    #[arg(name = "FILES", default_value = "-")]
    files: Vec<String>,
    /// print line numbers
    #[arg(short, long = "number")]
    number_lines: bool,
    /// print line numbers for non-blank lines
    #[arg(short = 'b', long = "number-nonblank", conflicts_with = "number_lines")]
    number_nonblank_lines: bool,
    /// show $ at the end of each line
    #[arg(short = 'E', long = "show-ends")]
    show_ends: bool,
}

type ProgResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> ProgResult<()> {
    let mut line_num = 0;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let (width, tab_char) = if config.number_lines | config.number_nonblank_lines {
                    (6, "\t")
                } else {
                    (0, "")
                };
                let line_end = if config.show_ends { "$" } else { "" };
                for line in file.lines() {
                    let line = line?;
                    let line_num_str = if config.number_lines
                        || (config.number_nonblank_lines && !line.is_empty())
                    {
                        line_num += 1;
                        line_num.to_string()
                    } else {
                        "".to_string()
                    };
                    if config.number_nonblank_lines && line.is_empty() {
                        println!("{}", line_end)
                    } else {
                        println!(
                            "{:>width$}{}{}{}",
                            line_num_str,
                            tab_char,
                            line,
                            line_end,
                            width = width
                        )
                    };
                }
            }
        }
    }
    Ok(()) // Return unit type in Ok variant to indicate success
}

pub fn get_args() -> ProgResult<Config> {
    let config = Config::parse();

    Ok(config)
}

fn open(filename: &str) -> ProgResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}