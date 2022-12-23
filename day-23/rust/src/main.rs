use std::collections::HashSet;

mod bbox;
mod util;
mod grid;

use grid::Grid;
use bbox::{Position};
use self::Direction::*;

fn parse_input(filename: &String) -> Vec<Position> {
    let mut positions = Vec::<Position>::new();
    for (row, s) in util::create_reader(&filename).enumerate() {
        for (column, p) in s.chars().enumerate() {
            if p == '#' {
                positions.push(Position{ row: row as i32,
                                         column: column as i32 });
            } else {
                assert!(p == '.');
            }
        }
    }
    return positions;
}

#[derive (Debug)]
enum Direction {
    North,
    South,
    West,
    East
}

fn check_dir(p: &Position, g: &Grid, d: &Direction) -> Option<Position> {
    match d {
        North =>
            if g.check_empty(&Position{ row: p.row - 1, column: p.column - 1 })
               && g.check_empty(&Position{ row: p.row - 1, column: p.column })
               && g.check_empty(&Position{ row: p.row - 1,
                                           column: p.column + 1 }) {
                Some(Position { row: p.row - 1, column: p.column })
            } else {
                None
            }
        South =>
            if g.check_empty(&Position{ row: p.row + 1, column: p.column - 1 })
               && g.check_empty(&Position{ row: p.row + 1, column: p.column })
               && g.check_empty(&Position{ row: p.row + 1,
                                           column: p.column + 1 }) {
                Some(Position{ row: p.row + 1, column: p.column })
            } else {
                None
            }
        West =>
            if g.check_empty(&Position{ row: p.row - 1, column: p.column - 1 })
               && g.check_empty(&Position{ row: p.row, column: p.column - 1})
               && g.check_empty(&Position{ row: p.row + 1,
                                           column: p.column - 1 }) {
                Some(Position { row: p.row, column: p.column - 1 })
            } else {
                None
            }
        East =>
            if g.check_empty(&Position{ row: p.row - 1, column: p.column + 1 })
               && g.check_empty(&Position{ row: p.row, column: p.column + 1})
               && g.check_empty(&Position{ row: p.row + 1,
                                           column: p.column + 1 }) {
                Some(Position { row: p.row, column: p.column + 1 })
            } else {
                None
            }
    }
}

fn check_neighborhood(p: &Position, g: &Grid) -> bool {
    for i in (p.row - 1)..(p.row + 2) {
        for j in (p.column - 1)..(p.column + 2) {
            if i == p.row && j == p.column {
                continue;
            }
            if !g.check_empty(&Position { row: i, column: j }) {
                return false;
            }
        }
    }
    return true;
}

#[derive (Debug, Clone)]
struct Transition {
    from: Position,
    to: Position,
}

#[derive (Debug, Clone)]
enum Change {
    Stay(Position),
    Move(Transition)
}

fn think(positions: &Vec<Position>, grid: &Grid, start_dir_index: usize)
    -> Vec<Change> {
    static DIRS: [Direction; 4] = [North, South, West, East];
    let mut changes = Vec::<Change>::new();
    for elf in positions {
        if check_neighborhood(&elf, &grid) {
            changes.push(Change::Stay(*elf));
            continue;
        }
        let mut found = false;
        for dir_index in 0..4 {
            let dir = &DIRS[(start_dir_index + dir_index) % 4];
            if let Some(np) = check_dir(&elf, &grid, dir) {
                changes.push(
                    Change::Move(Transition { from: *elf, to: np }));
                found = true;
                break;
            }
        }
        if !found {
            changes.push(Change::Stay(*elf));
        }
    }
    changes
}

fn resolve_conflicts(changes: &mut Vec<Change>) {
    let n = changes.len();
    let mut to_revert = HashSet::<usize>::new();
    for (i, c1) in changes.iter().enumerate() {
        if let Change::Move(p1) = c1 {
            for j in (i + 1)..n {
                if i == j {
                    continue;
                }
                if let Change::Move(p2) = &changes[j] {
                    if p1.to == p2.to {
                        // conflict
                        to_revert.insert(i);
                        to_revert.insert(j);
                    }
                }
            }
        }
    }
    for c in to_revert {
        if let Change::Move(trans) = &changes[c] {
            changes[c] = Change::Stay(trans.from);
        }
    }
}

fn apply_changes(positions: &mut Vec<Position>, changes: &Vec<Change>) {
    assert!(positions.len() == changes.len());
    for (i, c) in changes.iter().enumerate() {
        if let Change::Move(trans) = c {
            positions[i] = trans.to;
        }
    }
}

fn problem_1(mut positions: Vec<Position>) -> usize {
    let mut start_dir_index = 0;
    let mut grid = Grid::new(&positions);
    for _ in 0..10 {
        let mut changes = think(&positions, &grid, start_dir_index);
        resolve_conflicts(&mut changes);
        apply_changes(&mut positions, &changes);
        grid = Grid::new(&positions);
        start_dir_index += 1;
    }
    grid.empty_in_bbox_count()
}

fn count_transitions(changes: &Vec<Change>) -> usize {
    changes.iter().filter(|x| match x {
        Change::Move(_) => true,
        Change::Stay(_) => false
    }).count()
}

fn problem_2(mut positions: Vec<Position>) -> usize {
    let mut start_dir_index = 0;
    let mut grid = Grid::new(&positions);
    let mut round = 0;
    loop {
        let mut changes = think(&positions, &grid, start_dir_index);
        resolve_conflicts(&mut changes);
        let transitions_count = count_transitions(&changes);
        if transitions_count == 0 {
            break;
        }
        apply_changes(&mut positions, &changes);
        grid = Grid::new(&positions);
        start_dir_index += 1;
        round += 1;
    }
    round + 1
}

fn main() {
    let filename = util::parse_args();
    let positions = parse_input(&filename);

    println!("Day 23, problem 1: {}", problem_1(positions.clone()));
    println!("Day 23, problem 2: {}", problem_2(positions));
}
