use std::process;
use std::env;
use std::cmp;

mod valve;
use valve::Valve;

mod alone; // solves problem 1
mod together; // solves problem 2

fn parse_args() -> String {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Invalid arguments: I need the input file and nothing else");
        process::exit(1);
    }
    args.nth(1).unwrap()
}

fn floyd_warshall(valves: &Vec<Valve>) -> Vec::<Vec::<u32>> {
    let n = valves.len();
    let mut r = vec!(vec![1e8 as u32; n]; n);
    // Set the diagonal to zero and for each graph edge set distance to 1
    for i in 0..valves.len() {
        r[i][i] = 0;
        for j in &valves[i].connections {
            r[i][*j] = 1;
        }
    }
    // progressively update distances
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                r[i][j] = cmp::min(r[i][j], r[i][k] + r[k][j]);
            }
        }
    }
    r
}

fn main() {
    let filename = parse_args();
    let (valves, start_id) = valve::parse(&filename);
    let dists = floyd_warshall(&valves);
    let nonzero: Vec<usize> = valves.iter().enumerate()
        .filter(|x| x.1.rate > 0).map(|x| x.0).collect();

    println!("Day 16, problem 1: {}",
             alone::search(&valves, &nonzero, start_id, &dists));
    println!("Day 16, problem 2: {}",
             together::search(&valves, &nonzero, start_id, &dists));
}
