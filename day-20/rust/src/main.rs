use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;
use std::process;

fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Invalid arguments: I need the input file and nothing else");
        process::exit(1);
    }
    args.nth(1).unwrap()
}

fn parse_input(filename: &String) -> VecDeque<i64> {
    let file = File::open(&filename).unwrap_or_else(|_| {
        eprintln!("Cannot open file: {}", &filename);
        process::exit(1);
    });
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap()).collect()
}

fn simulate(nums: &mut VecDeque<i64>, indices: &mut VecDeque<usize>) {
    let n = nums.len();
    for i in 0..n {
        let idx = indices.iter().position(|&x| x == i).unwrap();
        if nums[idx] == 0 {
            continue;
        }
        nums.rotate_left(idx);
        indices.rotate_left(idx);
        let v = nums.pop_front().unwrap();
        let ii = indices.pop_front().unwrap();
        // note the (n - 1) is used because one element is popped out at this
        // moment. Correct results are generated only under this assumption.
        // This might be a kind of a bug in the problem description.
        let steps: usize = (v.abs() as usize) % (n - 1);
        if v > 0 {
            nums.rotate_left(steps);
            indices.rotate_left(steps);
        } else {
            nums.rotate_right(steps);
            indices.rotate_right(steps);
        }
        nums.push_front(v);
        indices.push_front(ii);
    }
}

fn find_grove_coordinates(nums: &VecDeque<i64>) -> i64 {
    let zero_index = nums.iter().position(|x| x == &0).unwrap();
    [1000, 2000, 3000].iter().map(|x| nums[(zero_index + x) % nums.len()]).sum()
}

fn problem_1(mut nums: VecDeque<i64>) -> i64 {
    let mut indices: VecDeque<usize> = (0..nums.len()).collect();
    simulate(&mut nums, &mut indices);
    find_grove_coordinates(&nums)
}

fn problem_2(mut nums: VecDeque<i64>) -> i64 {
    let mut indices: VecDeque<usize> = (0..nums.len()).collect();
    // apply the "decryption key"
    const KEY : i64 = 811589153;
    nums.iter_mut().for_each(|x| *x *= KEY);

    const ROUNDS: u32 = 10;
    for _ in 0..ROUNDS {
        simulate(&mut nums, &mut indices);
    }
    find_grove_coordinates(&nums)
}

fn main() {
    let filename = parse_args();
    let nums = parse_input(&filename);

    println!("Day 20, problem 1: {}", problem_1(nums.clone()));
    println!("Day 20, problem 2: {}", problem_2(nums));
}
