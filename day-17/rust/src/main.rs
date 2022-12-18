use std::env;
use std::fs;
use std::process;

mod game;
use game::{Move};

fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Invalid arguments: I need the input file and nothing else");
        process::exit(1);
    }
    args.nth(1).unwrap()
}

fn read_moves(filename: &String) -> Vec<Move>  {
    let data = fs::read_to_string(filename).expect("Failed to read input file");

    let moves: Result<Vec<Move>, String> = data
        .trim().chars().filter(|x| x != &'\n')
        .map( |x| {
            match &x {
                '<' => Ok(Move::Left),
                '>' => Ok(Move::Right),
                _ => Err(format!("Invalid input: {x}"))
            }
        }).collect();

    moves.unwrap_or_else(|err| {
        eprintln!("Parser error while reading {}: {}", &filename, err);
        process::exit(1);
    })
}

fn main() {
    let filename = parse_args();
    let moves = read_moves(&filename);
    let r1 = game::simulate(moves, 2022);
    let moves2 = read_moves(&filename);
    let r2 = game::simulate(moves2, 1000000000000);

    println!("Day 17, problem 1: {}", r1);
    println!("Day 17, problem 1: {}", r2);
}
