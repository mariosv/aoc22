use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;
use std::collections::VecDeque;

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

type Stack = VecDeque<char>;

fn parse_stacks(lines: &mut impl Iterator<Item = String>) -> [Stack; 9] {
    let mut stacks: [Stack; 9] = [(); 9].map(|_| Stack::new());
    for line in lines {
        let s = line.trim_end();
        if (&s).is_empty() || (&s).trim_start().chars().next().unwrap() != '[' {
            break;
        }
        for i in 0..9 {
            let index = 3 * i + i + 1;
            if index < (&s).chars().count() {
                let c: char = (&s).chars().nth(index).unwrap();
                if c != ' ' {
                    stacks[i].push_back(c);
                }
            }
        }
    }
    return stacks;
}

struct Move {
    quantity: u32,
    from: u32,
    to: u32
}

impl FromIterator<u32> for Move {
    fn from_iter<I: IntoIterator<Item=u32>>(iter: I) -> Self {
        //let mut c = Mov::new();
        let mut it = iter.into_iter();
        let q = (&mut it).next().unwrap();
        let f = (&mut it).next().unwrap();
        let t = (&mut it).next().unwrap();
        Move { quantity: q, from: f, to: t }
    }
}

fn parse_move_line(s: String) -> Move {
    let mut it = s.split(' ');
    let q: u32 = it.nth(1).unwrap().parse().unwrap();
    let f: u32 = it.nth(1).unwrap().parse().unwrap();
    let t: u32 = it.nth(1).unwrap().parse().unwrap();
    Move { quantity: q, from: f, to: t }
}

fn parse_moves(lines: &mut impl Iterator<Item = String>) -> Vec<Move> {
    lines.filter(|x| !x.is_empty())
         .map(parse_move_line)
         .collect::<Vec<Move>>()
}

fn parse_input(filename: &String) -> ([Stack; 9], Vec<Move>) {
    let mut reader = create_reader(&filename);
    let stacks = parse_stacks(&mut reader);
    let moves = parse_moves(&mut reader);

    return (stacks, moves);
}

fn rearange_9000(stacks: &[Stack; 9], moves: &Vec<Move>) -> [Stack; 9] {
    let mut s = stacks.clone();
    for m in moves {
        for _ in 0..m.quantity {
            let el = s[m.from as usize - 1].pop_front().unwrap();
            s[m.to as usize - 1].push_front(el);
        }
    }
    s
}

fn rearange_9001(stacks: &[Stack; 9], moves: &Vec<Move>) -> [Stack; 9] {
    let mut s = stacks.clone();
    for m in moves {
        let drained = s[m.from as usize - 1]
                              .drain(..(m.quantity as usize))
                              .collect::<VecDeque<_>>();
        for t in (&drained).into_iter().rev() {
            s[m.to as usize - 1].push_front(*t);
        }
    }
    s
}

fn create_result_string(stacks: &[Stack; 9]) -> String {
    stacks.iter().map(|x| { x.front().unwrap() }).collect()
}

fn main() {
    let filename = parse_args();
    let (stacks, moves) = parse_input(&filename);

    let p1 = rearange_9000(&stacks, &moves);
    let p1_result = create_result_string(&p1);
    println!("Day 5, problem 1: {}", p1_result);

    let p2 = rearange_9001(&stacks, &moves);
    let p2_result = create_result_string(&p2);
    println!("Day 5, problem 2: {}", p2_result)
}
