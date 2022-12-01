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

/// Parses the contents of 'filename', calculates the total calories per unit
/// and sums the 'n' higher values.
/// Uses a heap data structure to store the n maximum values.
fn parse_and_find_max_n_sum(filename: &String, n: usize) -> i32 {
    let mut max = BinaryHeap::new();
    if let Ok(lines) = read_lines(&filename) {
        let mut s: i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    if max.len() < n {
                        max.push(-s);
                    } else {
                        if let Some(current_min) = max.peek() {
                            if s > *current_min {
                                max.push(-s);
                                max.pop();
                            }
                        }
                    }
                    s = 0;
                } else {
                    s += ip.parse::<i32>().unwrap();
                }
            }
        }
    } else {
        panic!("Error while reading file: {}", &filename);
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
