use std::env;
use std::fs;
use std::process;
use std::collections::VecDeque;

fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Invalid arguments: I need the input file and nothing else");
        process::exit(1);
    }
    args.nth(1).unwrap()
}

/// nlogn implementation
fn check_unique_chars(seq: &VecDeque<char>) -> bool {
    let mut v = Vec::from(seq.clone());
    v.sort();
    v.dedup();
    v.len() == seq.len()
}

fn find_sequence(data: &String, length: u32) -> Result<u32, String> {
    let mut seq = VecDeque::<char>::new();
    let mut n = 0;
    for c in data.chars() {
        n += 1;
        seq.push_back(c);
        if seq.len() > length as usize {
            seq.pop_front();
        }
        if seq.len() == length as usize && check_unique_chars(&seq) {
            return Ok(n);
        }
    }
    Err(format!("A sequence with {} unique characters was not found.", length))
}

fn read_file(filename: &String) -> String {
    fs::read_to_string(&filename).unwrap_or_else(|_| {
        eprintln!("Cannot open file: {}", &filename);
        process::exit(1);
    })
}

fn main() {
    let filename = parse_args();
    let data = read_file(&filename);

    let mut i = 0;
    for length in [4, 14] {
        i += 1;
        print!("Day 6, problem {}: ", i);
        let p = find_sequence(&data, length);
        match p {
            Ok(index) => println!("{}", index),
            Err(msg) => println!("Error: {} ", msg)
        }
    }
}
