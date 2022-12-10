use std::env;
use std::fs;
use std::process;
use itertools::Itertools;

fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Invalid arguments: I need the input file and nothing else");
        process::exit(1);
    }
    args.nth(1).expect("Already checked")
}

fn read_grid(filename: &String) -> (Vec<u32>, usize)  {
    let data = fs::read_to_string(filename).expect("Failed to read input file");
    let raw: Vec<u32>
        = data.trim().chars().filter(|x| x != &'\n')
            .map(|x| x.to_digit(10).unwrap()).collect();
    let n = (raw.len() as f64).sqrt() as usize;
    if n * n != raw.len() {
        eprintln!("Invalid data dimensions");
        process::exit(1);
    }
    (raw, n)
}

fn check_visibility(grid: &Vec<&[u32]>,
                    row: usize,
                    column: usize,
                    n: usize) -> bool {
    if row == 0 || column == 0 || row == n - 1 || column == n - 1 {
        return true;
    }
    let v = grid[row][column];
    let h1 = grid[0..row].iter()
        .map(|x| x[column])
        .filter(|x| x >= &v).count() == 0;
    let h2 = grid[row + 1..n].iter().map(|x| x[column])
        .filter(|x| x >= &v).count() == 0;
    let v1 = grid[row][0..column].iter().filter(|x| x >= &&v).count() == 0;
    let v2 = grid[row][column + 1..n].iter().filter(|x| x >= &&v).count() == 0;

    h1 || h2 || v1 || v2
}

fn problem_1(data: &Vec<u32>, n: usize) -> u32 {
    let grid: Vec<&[u32]> = data.chunks(n).collect();
    let mut count = 0;
    for i in 0..n {
        for j in 0..n {
            if check_visibility(&grid, i, j, n) {
                count += 1;
            }
        }
    }
    count
}

fn obstacle_distance(grid: &Vec<&[u32]>,
                     row: usize,
                     column: usize,
                     n: usize) -> u32 {
    let v = grid[row][column];

    let h1 = if let Some(t) = grid[0..row].iter().map(|x| x[column]).rev()
        .find_position(|x| x >= &v) { t.0 + 1 } else { row };
    let h2 = if let Some(t) = grid[row + 1..n].iter().map(|x| x[column])
        .find_position(|x| x >= &v) { t.0 + 1 } else { n - row - 1 };
    let v1 = if let Some(t) = grid[row][0..column].iter().rev()
        .find_position(|x| *x >= &v) { t.0 + 1 } else { column };
    let v2 = if let Some(t) = grid[row][(column + 1)..n].iter()
        .find_position(|x| *x >= &v) { t.0 + 1 } else { n - column - 1};

    (h1 * h2 * v1 * v2).try_into().unwrap()
}

fn problem_2(data: &Vec<u32>, n: usize) -> u32 {
    let grid: Vec<&[u32]> = data.chunks(n).collect();
    let mut r = 0;
    for i in 0..n {
        for j in 0..n {
            if check_visibility(&grid, i, j, n) {
                r = std::cmp::max(r, obstacle_distance(&grid, i, j, n))
            }
        }
    }
    r
}

fn main() {
    let filename = parse_args();
    let (data, n) = read_grid(&filename);
    println!("Day 8, problem 1: {}", problem_1(&data, n));
    println!("Day 8, problem 2: {}", problem_2(&data, n));
}
