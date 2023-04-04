use std::error::Error;
use clap::{App,Arg,ArgGroup};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config{
    files:Vec<String>,
    number_lines:bool,
    number_nonblank_lines:bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

// Function to get arguments
pub fn get_args()->MyResult<Config>{
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Nurudeen Abdul-Karim <nurudeen.karim2016@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Paths to files")
                .default_value("-")
                .min_values(1)
        )
        .arg(
            Arg::with_name("number_lines")
                .help("Number lines")
                .takes_value(false)
                .short("n")
                .long("number")
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .help("Number only non-blank lines")
                .takes_value(false)
                .short("b")
                .long("number-nonblank")
        )
        .group(
            ArgGroup::with_name("number")
                .args(&["number_lines","number_nonblank_lines"])
        ).get_matches();

        Ok(Config{
            files:matches.values_of_lossy("files").unwrap(),
            number_lines:matches.is_present("number_lines"),
            number_nonblank_lines:matches.is_present("number_nonblank_lines"),
        })
}

fn open(filename:&str)->MyResult<Box<dyn BufRead>>{
    match filename{
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config:Config)->MyResult<()>{
    let mut code = 0;
    for filename in config.files{
        match open(&filename){
            Err(err) => {
                eprintln!("catr: {}: {}",filename,err);
                code += 1;
            },
            Ok(file) => {
                for line_result in file.lines(){
                    let line = line_result?;
                    println!("{}",line);
                }
            },
        }
    }
    if code > 0 {
        std::process::exit(code);
    }
    Ok(())
}
