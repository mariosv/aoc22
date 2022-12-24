use std::fmt;

use crate::util;

#[derive (Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub row: usize,
    pub column: usize
}

#[derive (Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<'
        })
    }
}

#[derive (Clone)]
pub enum Cell {
    Blizzards(Vec<Direction>),
    Wall,
}

pub type Grid = Vec::<Vec<Cell>>;

pub fn print(grid: &Grid, actor: &Position, time: u32) {
    let mut s: String = format!("Time: {}\n", time);
    for (i, row) in grid.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if i == actor.row && j == actor.column {
                if let Cell::Blizzards(v) = el {
                    if v.len() == 0 {
                        s += "E";
                        continue;
                    } else {
                        panic!("We are hit by a blizzard!");
                    }
                } else {
                    panic!("Invalid actor position: {}, {}", i, j);
                }
            }
            match el {
                Cell::Wall => { s += "#"; },
                Cell::Blizzards(blizzards) => {
                    if blizzards.len() == 1 {
                        s += &format!("{}", blizzards[0]);
                    } else if blizzards.len() == 0 {
                        s += ".";
                    } else {
                        s += &format!("{}", blizzards.len());
                    }
                }
            }
        }
        if i != (grid.len() - 1) {
            s += "\n";
        }
    }
    println!("{}", s);
}

pub fn parse(filename: &String) -> Grid {
    util::create_reader(&filename).map(|x| {
        x.trim().chars().map(|y| match y {
            '#' => Cell::Wall,
            '.' => Cell::Blizzards(Vec::<Direction>::new()),
            _ => {
                let dir = match y {
                    '^' => Ok(Direction::Up),
                    '>' => Ok(Direction::Right),
                    'v' => Ok(Direction::Down),
                    '<' => Ok(Direction::Left),
                    _ =>  Err(format!("Unexpected character: {}", y))
                }.unwrap();
                let mut v = Vec::<Direction>::new();
                v.push(dir);
                Cell::Blizzards(v)
            }
        }).collect::<Vec<Cell>>()
    }).collect()
}

fn find_free_node_on_row(g: &Grid, row: usize) -> Position {
    let n = g[0].len();
    let mut p = Position { row: row, column: n };
    for j in 0..g[p.row].len() {
        if let Cell::Blizzards(_) = &g[p.row][j] {
            p.column = j;
        }
    }
    if p.column == n {
        panic!("Failed to locate start/exit nodes");
    }
    p
}

pub fn find_start_position(grid: &Grid) -> Position {
    find_free_node_on_row(&grid, 0)
}

pub fn find_exit_position(grid: &Grid) -> Position {
    find_free_node_on_row(&grid, grid.len() - 1)
}
