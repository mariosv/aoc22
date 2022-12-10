use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

const SCREEN_WIDTH: i32 = 40;

type Command = Option<i32>;

fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Invalid arguments: I need the input file and nothing else");
        process::exit(1);
    }
    args.nth(1).expect("Already checked")
}

struct CpuState<T> where T: Iterator<Item = Command> {
    iter: T,
    acc: i32,
    to_add: Option<i32>
}

impl<T> Iterator for CpuState<T> where T: Iterator<Item = Command> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        match self.to_add {
            Some(v) => {
                let old: i32 = self.acc;
                self.acc += v;
                self.to_add = None;
                Some(old)
            },
            None => {
                match self.iter.next() {
                    None => None,
                    Some(cmd) => {
                        if cmd.is_some() {
                            self.to_add = cmd;
                        }
                        Some(self.acc)
                    }
                }
            }
        }
    }
}

// Create a cpu-state sequence generator
fn cpu_state(iter_: impl Iterator<Item = Command>)
    -> CpuState<impl Iterator<Item = Command>> {
    CpuState { iter: iter_, acc: 1, to_add: None }
}

fn create_reader(filename: &String) -> impl Iterator<Item = Command> {
    let file = File::open(&filename).unwrap_or_else(|_| {
        eprintln!("Cannot open file: {}", &filename);
        process::exit(1);
    });
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().split_whitespace().map(|x| x.to_string())
                  .collect::<Vec<_>>())
        .map(|x| {
            let mut it = x.iter();
            it.next().unwrap();
            match it.next() {
                None => None,
                Some(v) => Some(v.parse::<i32>().unwrap())
            }
        })
}

fn problem_1(mut cpu: CpuState<impl Iterator<Item = Command>>) -> i32 {
    let mut r: i32 = cpu.nth(19).unwrap() * 20;
    for i in 1..6 {
        let mut v = 0;
        for _ in 0..40 {
            v = cpu.next().unwrap();
        }
        r += (20 + i * 40) * v;
    }
    r
}

fn print_pixel(screen_pos: i32, acc: i32) {
    let sx = screen_pos % SCREEN_WIDTH;
    if sx == 0 {
        print!("\n");
    }
    if sx >= acc - 1 && sx <= acc + 1 {
        print!("#");
    } else {
        print!(".");
    }
}

fn problem_2(cpu: CpuState<impl Iterator<Item = Command>>) {
    println!("Day 10, problem 2:");
    let mut screen_pos = 0;
    for acc in cpu {
        print_pixel(screen_pos, acc);
        screen_pos += 1
    }
    println!("");
}

fn main() {
    let filename = parse_args();
    println!("Day 10, problem 1: {}",
             problem_1(cpu_state(create_reader(&filename))));
    problem_2(cpu_state(create_reader(&filename)));
}
