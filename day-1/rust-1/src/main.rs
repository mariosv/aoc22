use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BinaryHeap;
use std::env;

fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("I need the input file and nothing else");
    }
    args.nth(1).expect("Already checked")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(filename: &String) -> impl Iterator<Item = String> {
    let lines = read_lines(&filename)
        .unwrap_or_else(|e| panic!("Error while reading input: {}", e))
        .map(|x| { match x {
            Ok(s) => s,
            Err(e) => panic!("Error while parsing codes: {}", e)
        }
    });
    lines
}

/// Parses the contents of 'filename', calculates the total calories per unit
/// and sums the 'n' higher values.
/// Uses a heap data structure to store the n maximum values.
fn parse_and_find_max_n_sum(filename: &String, n: usize) -> i32 {
    let mut max = BinaryHeap::new();
    let mut lines = parse_input(&filename).peekable();
    while lines.peek().is_some() {
        let calories : i32 = lines
                       .by_ref()
                       .take_while(|s| !s.is_empty())
                       .map(|s| s.parse::<i32>().unwrap())
                       .sum();
        if max.len() < n {
            max.push(calories);
        } else {
            if let Some(current_min) = max.peek() {
                if calories > *current_min {
                    max.push(-calories);
                    max.pop();
                }
            }
        }
    }
    -max.iter().sum::<i32>()
}

fn main() {
    let filename = parse_args();
    let p1 = parse_and_find_max_n_sum(&filename, 1);
    println!("Day 1, problem 1: {}", p1);
    let p2 = parse_and_find_max_n_sum(&filename, 3);
    println!("Day 2, problem 2: {}", p2);
}
