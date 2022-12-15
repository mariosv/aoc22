use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

mod parser;
mod packet;
use packet::Packet;

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

fn parse_input(filename: &String) -> Vec<Packet> {
    create_reader(&filename)
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .map(|x| parser::parse(&x).unwrap())
        .collect()
}

fn problem_1(packets: &Vec<Packet>) -> usize {
    packets.chunks(2).enumerate().filter(|x| x.1[0] <= x.1[1]).map(|x| x.0 + 1)
           .sum()
}

fn problem_2(packets: &mut Vec<Packet>) -> usize {
    let ba: Packet = Packet::List(vec!(Packet::List(vec!(Packet::Num(2)))));
    let bb: Packet = Packet::List(vec!(Packet::List(vec!(Packet::Num(6)))));
    packets.push(ba.clone());
    packets.push(bb.clone());
    packets.sort();
    let a = packets.iter().position(|x| x == &ba).unwrap() + 1;
    let b = packets.iter().position(|x| x == &bb).unwrap() + 1;
    a *
}

fn main() {
    let filename = parse_args();
    let mut packets = parse_input(&filename);

    println!("Day 13, problem 1: {}", problem_1(&packets));
    println!("Day 13, problem 2: {}", problem_2(&mut packets));
}
