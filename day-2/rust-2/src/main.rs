use std::fs::File;
use std::env;
use std::io::{self, BufRead};

/// Codified game rules. Rows and columns represent the choices of the other
/// player and you respectively in the order Rock-Paper-Scissors and the matrix
/// value is the outcome (Lose-Draw-Win)
const RULES : [[u32; 3]; 3] = [
    [1, 2, 0],
    [0, 1, 2],
    [2, 0, 1]
];

fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("I need the input file and nothing else");
    }
    args.nth(1).unwrap()
}

fn read_lines(filename: &String) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn symbol_to_id(s: &str) -> u32 {
    let mut r = 2;
    if s == "A" || s == "X" {
        r = 0;
    } else if s == "B" || s == "Y" {
        r = 1;
    }
    r
}

fn parse_codes(line: String) -> (u32, u32) {
    let t: Vec<_> = line
                    .split(' ')
                    .map(symbol_to_id)
                    .collect();
    (t[0], t[1])
}

fn score(you: u32, outcome: u32) -> u32 {
    you + 1 + outcome * 3
}

// Returns the score of a round based on the initial (problem-1) interpretation
// of the input pairs
fn play(input: (u32, u32)) -> u32 {
    let (other, you) = input;
    let outcome = RULES[other as usize][you as usize];
    score(you, outcome)
}

// Returns the score of a round based on the corrected (problem-2)
// interpretation of the input pairs
fn play_corrected(input: (u32, u32)) -> u32 {
    let (other, outcome) = input;
    let you = RULES[other as usize].iter().position(|&x| x == outcome).unwrap();
    score(you as u32, outcome)
}

fn parse_input(filename: &String) -> impl Iterator<Item = (u32, u32)> {
    let lines = read_lines(&filename)
        .unwrap_or_else(|e| panic!("Error while reading input: {}", e))
        .map(|x| { match x {
            Ok(s) => parse_codes(s),
            Err(e) => panic!("Error while parsing codes: {}", e)
        }
    });
    lines
}

fn main() {
    let filename = parse_args();

    let problem_1 = parse_input(&filename)
                    .map(play)
                    .fold(0, |a, b| { a + b });
    let problem_2 = parse_input(&filename)
                    .map(play_corrected)
                    .fold(0, |a, b| { a + b });

    println!("Day 2, problem 1: {}", problem_1);
    println!("Day 2, problem 2: {}", problem_2);
}
