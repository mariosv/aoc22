use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;
use std::collections::HashMap;

fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Invalid arguments: I need the input file and nothing else");
        process::exit(1);
    }
    args.nth(1).unwrap()
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

fn change_dir(target: &str, dirs: &mut Vec::<String>) {
    if target == ".." {
        assert!(!dirs.is_empty());
        dirs.pop().unwrap();
    } else {
        let mut name = target.to_string();
        if dirs.len() > 1 {
            name = dirs[1..].join("/") + &name
        }
        dirs.push(name);
    }
}

fn sum_file_sizes(reader: &mut impl Iterator<Item = String>)
    -> (u64, Option<String>) {
    let mut file_sum: u64 = 0;
    let mut line = reader.next();
    while line.is_some() {
        let s = line.as_ref().unwrap();
        if s.starts_with("$") {
            break;
        }
        let mut p = s.split_whitespace();
        let a = p.next().unwrap();
        if a != "dir" {
            file_sum +=  a.parse::<u64>().unwrap();
        }
        line = reader.next();
    }
    (file_sum, line)
}

fn find_dir_sizes(filename: &String) -> HashMap::<String, u64> {
    let mut reader = create_reader(filename);
    let mut dir_sizes = HashMap::<String, u64>::new();
    let mut dirs = Vec::<String>::new();
    let mut line = reader.next();
    while line.is_some() {
        let s = line.as_ref().unwrap();
        assert!(&s.starts_with("$"));
        let mut p = s.split_whitespace();
        let cmd = p.nth(1).unwrap();
        if cmd == "cd" {
            let nd = p.next().unwrap();
            change_dir(&nd, &mut dirs);
            line = reader.next();
        } else {
            let (file_sum, nline) = sum_file_sizes(&mut reader);
            line = nline;
            // ls
            for d in &dirs {
                if let Some(x) = dir_sizes.get_mut(d) {
                    *x += file_sum;
                } else {
                    dir_sizes.insert(d.clone(), file_sum);
                }
            }
        }
    }
    dir_sizes
}

fn problem_1(dir_sizes: &HashMap::<String, u64>) -> u64 {
    dir_sizes.iter().map(|x| x.1).filter(|x| *x < &100000).sum()
}

fn problem_2(dir_sizes: &HashMap::<String, u64>) -> u64 {
    const TOTAL: u64 = 70000000;
    const NEEDED: u64 = 30000000;
    let tofree = NEEDED - (TOTAL - dir_sizes.get("/").unwrap());
    *dir_sizes.iter().map(|x| x.1).filter(|x| *x > &tofree).min().unwrap()
}

fn main() {
    let filename = parse_args();
    let dir_sizes = find_dir_sizes(&filename);

    println!("Day 7, problem 1: {}", problem_1(&dir_sizes));
    println!("Day 7, problem 2: {}", problem_2(&dir_sizes));
}
