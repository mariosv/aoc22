use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;
use std::collections::HashSet;

mod monkey;
use monkey::Monkey;

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


fn parse_input(filename: &String) -> Vec<Monkey> {
    let mut lines = create_reader(&filename);
    let mut data = Vec::<Monkey>::new();
    loop {
        data.push(Monkey::parse(&mut lines));
        match lines.next() {
            Some(s) => {
                assert!(s.trim().len() == 0);
            },
            None => { break; }
        }
    }
    data
}

fn simulate(monkeys: &mut Vec<Monkey>,
            rounds: u32,
            relax: impl Fn(u64) -> u64) -> u64 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut m = &mut monkeys[i];
            while let Some(mut el) = m.items.pop_front() {
                m.activity += 1;
                el = m.inspect(el);
                el = relax(el);
                let target_index = m.forward(el);
                m = &mut monkeys[target_index];
                m.items.push_back(el);
                m = &mut monkeys[i];
            }
        }
    }
    monkeys.sort_by(|a, b| a.activity.cmp(&b.activity));
    monkeys.iter().rev().take(2).map(|x| x.activity as u64).product()
}

fn problem_2(monkeys: &mut Vec<Monkey>) -> u64 {
    let divs: HashSet<u64> = monkeys.iter().map(|x| x.divisible_by).collect();
    let relax_factor: u64 = divs.iter().product();
    simulate(&mut monkeys.clone(), 10000, |x| x  % relax_factor)
}

fn main() {
    let filename = parse_args();
    let monkeys = parse_input(&filename);

    println!("Day 11, problem 1: {}",
             simulate(&mut monkeys.clone(), 20, |x| x / 3));
    println!("Day 11, problem 2: {}", problem_2(&mut monkeys.clone()));
}
