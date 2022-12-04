use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;
use std::collections::HashSet;

fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("I need the input file and nothing else");
    }
    args.nth(1).expect("Already checked")
}

fn score(c: char) -> u32 {
    if c.is_lowercase() {
        return c as u32 - 'a' as u32 + 1;
    }
    c as u32 - 'A' as u32 + 27
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

fn find_common_char(s: String) -> char {
    let (a, b) = &s.split_at(&s.len() / 2);
    a.chars().filter(|x| b.contains(*x)).next().unwrap()
}

fn problem_1(filename: &String) -> u32 {
    let lines = create_reader(filename);
    lines.map(find_common_char)
         .map(score)
         .sum()
}

fn problem_2(filename: &String) -> u32 {
    let mut lines = create_reader(filename).peekable();
    let mut s: u32 = 0;
    while lines.peek().is_some() {
        let group = lines.by_ref().take(3);
        let mut i: u32 = 0;
        let mut set = HashSet::<char>::new();
        for item in group {
            if 0 == i {
                for c in item.chars() {
                    set.insert(c);
                }
            } else {
                let mut new_set = HashSet::<char>::new();
                for c in item.chars() {
                    if set.contains(&c) {
                        new_set.insert(c);
                    }
                }
                set = new_set;
            }
            i += 1;
        }
        s += score(*set.iter().next().unwrap());
    }
    s
}

fn main() {
    let filename = parse_args();
    println!("Day 3, problem 1: {}", problem_1(&filename));
    println!("Day 3, problem 2: {}", problem_2(&filename));
}
