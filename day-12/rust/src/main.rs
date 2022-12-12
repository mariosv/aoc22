use std::env;
use std::fs::File;
use std::cmp;
use std::io::{self, BufRead};
use std::process;
use petgraph::graph::{NodeIndex, Graph};
use petgraph::algo::dijkstra;

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

struct Node {
    i: usize,
    j: usize
}

fn to_code(c: char) -> u32 {
    c as u32 - 'a' as u32
}

fn check_columns(grid: &Vec<Vec<u32>>) -> usize {
    assert!(!grid.is_empty());
    let columns = grid[0].len();
    for i in 1..grid.len() {
        if columns != grid[i].len() {
            panic!("Invalid columns in line: {}", i + 1);
        }
    }
    columns
}

fn parse_input(filename: &String) -> (Vec<Vec<u32>>, Node, Node, usize) {
    let lines = create_reader(filename);
    let mut start: Option<Node> =  None;
    let mut end: Option<Node> = None;
    let mut grid = Vec::<Vec<u32>>::new();
    let mut i = 0;
    for line in lines {
        grid.push(Vec::new());
        for (j, c) in line.trim().chars().enumerate() {
            let v: u32 = match c {
                'S' => {
                    start = Some(Node{ i: i, j: j });
                    to_code('a')
                },
                'E' => {
                    end = Some(Node{ i: i, j: j });
                    to_code('z')
                },
                _ => to_code(c)
            };
            grid[i].push(v);
        }
        i += 1;
    }
    let columns = check_columns(&grid);
    if !start.is_some() || !end.is_some() {
        eprintln!("Invalid input. Start of finish notes are not annotated");
        process::exit(1);
    }
    (grid, start.unwrap(), end.unwrap(), columns)
}

fn to_idx(i: usize, j: usize, cols: usize) -> usize {
    i * cols + j
}

/// Converts the given grid to a graph based on the transition rules for hill
/// climbing
fn to_graph(grid: &Vec<Vec<u32>>, cols: usize) -> Graph<u32, u32> {
    let mut g = Graph::<u32, u32>::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let idx = to_idx(i, j, cols) as u32;
            let v = grid[i][j];
            let mut new_edges = Vec::<(u32, u32)>::new();
            if i != 0 && grid[i - 1][j]  < v + 2 {
                new_edges.push((idx, to_idx(i - 1, j, cols) as u32));
            }
            if i != (grid.len() - 1) && grid[i + 1][j] < v + 2 {
                new_edges.push((idx, to_idx(i + 1, j, cols) as u32));
            }
            if j != 0 && grid[i][j - 1] < v + 2 {
                new_edges.push((idx, to_idx(i, j - 1, cols) as u32));
            }
            if j != (cols - 1) && grid[i][j + 1] < v + 2 {
                new_edges.push((idx, to_idx(i, j + 1, cols) as u32));
            }
            g.extend_with_edges(new_edges);
        }
    }
    g
}

fn to_node_index(node: &Node, cols: usize) -> NodeIndex {
    (to_idx(node.i, node.j, cols) as u32).try_into().unwrap()
}

fn problem_1(g: &Graph<u32, u32>, cols: usize, start: &Node, end: &Node)
    -> u32 {
    let start_node = to_node_index(start, cols);
    let end_node = to_node_index(end, cols);
    let res = dijkstra(g, start_node, Some(end_node), |_| 1);
    res[&end_node]
}

fn problem_2(grid: &Vec<Vec<u32>>,
             g: &mut Graph<u32, u32>,
             cols: usize,
             end: &Node) -> u32 {
    // reverse the direction of all the edges of the graph and set the "end"
    // node as the starting node for dijkstra
    g.reverse();
    let end_node = to_node_index(&end, cols);
    let res = dijkstra(&*g, end_node, None, |_| 1);
    // find the node with value 'a' (==0) with the minimum distance from "end"
    let mut min = u32::MAX;
    for i in 0..grid.len() {
        for j in 0..cols {
            let node = &to_node_index(&Node { i: i, j: j }, cols);
            // Note that the dijkstra result may not contain a node index if the
            // corresponding node is unreachable
            if res.contains_key(node) && grid[i][j] == 0 {
                min = cmp::min(min, res[node]);
            }
        }
    }
    min
}

fn main() {
    let filename = parse_args();
    let (grid, start, end, cols) = parse_input(&filename);
    let mut g = to_graph(&grid, cols);
    println!("Day 12, problem 1: {}", problem_1(&g, cols, &start, &end));
    println!("Day 12, problem 2: {}", problem_2(&grid, &mut g, cols, &end));
}
