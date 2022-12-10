use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;
use std::collections::HashSet;

fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Invalid arguments: I need the input file and nothing else");
        process::exit(1);
    }
    args.nth(1).unwrap()
}

fn create_reader(filename: &String) -> impl Iterator<Item = (char, u32)> {
    let file = File::open(&filename).unwrap_or_else(|_| {
        eprintln!("Cannot open file: {}", &filename);
        process::exit(1);
    });
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap()
                  .split_whitespace()
                  .map(|x| x.to_string())
                  .collect::<Vec<String>>())
        .map(|x| {
            let mut it = x.iter();
            let m = it.next().unwrap().chars().next().unwrap();
            let q = it.next().unwrap().parse::<u32>().unwrap();
            (m, q)
        })
}

fn head_move(m: char, p: &mut [i32; 2]) {
    if m == 'R' {
        p[1] += 1;
    } else if m == 'L' {
        p[1] -= 1;
    } else if m == 'U' {
        p[0] += 1;
    } else {
        p[0] -= 1;
    }
}

fn follow_dir(h: i32, t: i32) -> i32 {
    if (h - t) == 2 {
        return t + 1;
    }
    if (t - h) == 2 {
        return t - 1;
    }
    return t
}

fn follow(prev: [i32; 2], node: &mut [i32; 2]) {
    if (prev[0] - node[0]).abs() < 2 && (prev[1] - node[1]).abs() < 2 {
        return;
    }
    let mut nti = follow_dir(prev[0], node[0]);
    let mut ntj = follow_dir(prev[1], node[1]);

    if (prev[0] - node[0]).abs() == 2 && (prev[1] - node[1]).abs() == 1 {
        ntj = prev[1];
    }
    if (prev[0] - node[0]).abs() == 1 && (prev[1] - node[1]).abs() == 2 {
        nti = prev[0];
    }
    node[0] = nti;
    node[1] = ntj;
}

fn simulate(moves: impl Iterator<Item = (char, u32)>, nodes: usize) -> usize {
    let mut p = vec![[0 as i32; 2]; nodes];
    let mut tail_pos = HashSet::<[i32; 2]>::new();
    for (m, q) in moves {
        for _ in 0..q {
            head_move(m, &mut p[0]);
            for j in 1..nodes {
                follow(p[j - 1], &mut p[j]);
                if j == nodes - 1 {
                    tail_pos.insert(p[j]);
                }
            }
        }
    }
    tail_pos.len()
}

fn main() {
    let filename = parse_args();

    println!("Day 9, problem 1: {}", simulate(create_reader(&filename), 2));
    println!("Day 9, problem 2: {}", simulate(create_reader(&filename), 10));
}
