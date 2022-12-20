use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Invalid arguments: I need the input file and nothing else");
        process::exit(1);
    }
    args.nth(1).unwrap()
}

pub fn create_reader(filename: &String) -> impl Iterator<Item = String> {
    let file = File::open(&filename).unwrap_or_else(|_| {
        eprintln!("Cannot open file: {}", &filename);
        process::exit(1);
    });
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
}
