use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

type Range = (u32, u32);

fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Invalid arguments: I need the input file and nothing else");
        process::exit(1);
    }
    args.nth(1).expect("Already checked")
}

fn create_reader(filename: &String) -> impl Iterator<Item = String> {
    let file = File::open(&filename).unwrap_or_else(|_| {
        eprintln!("Cannot open file: {}", &filename);
        process::exit(1);
    });
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
}

fn parse_range(s: &str) -> Range {
    let v: Vec<u32> = s.split('-')
     .map(|x| x.parse::<u32>().unwrap())
     .collect();
    ((&v)[0], (&v)[1])
}


fn parse_line(line: String) -> (Range, Range) {
    let t: Vec<_> = line.split(',').map(parse_range).collect();
    (t[0], t[1])
}

/// Problems 1 and 2 of day-4 share the same structure. Their only difference
/// is the predicate that is used which is passed as a function
fn common(filename: &String, predicate: fn(Range, Range) -> bool) -> u32 {
    create_reader(filename)
        .map(parse_line)
        .filter(|x| predicate(x.0, x.1))
        .count()
        .try_into() // usize to u32
        .unwrap()
}

fn contains(a: Range, b: Range) -> bool {
    b.0 >= a.0 && b.1 <= a.1
}

fn predicate_problem_1(a: Range, b: Range) -> bool {
    contains(a, b) || contains(b, a)
}

fn predicate_problem_2(a: Range, b: Range) -> bool {
    !(a.0 > b.1 || b.0 > a.1)
}

fn main() {
    let filename = parse_args();
    println!("Day 4, problem 1: {}", common(&filename, predicate_problem_1));
    println!("Day 4, problem 2: {}", common(&filename, predicate_problem_2));
}
