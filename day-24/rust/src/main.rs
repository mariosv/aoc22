use std::collections::VecDeque;

mod util;
mod scene;
use scene::{Direction, Grid, Cell, Position};

fn step(dir: &Direction, row: usize, column: usize) -> Position {
    let mut pos = Position { row, column };
    match dir {
        Direction::Up => { pos.row -= 1; },
        Direction::Right => { pos.column += 1; },
        Direction::Down => { pos.row += 1; },
        Direction::Left => { pos.column -= 1; },
    }
    pos
}

fn reenter(pos: &mut Position, dir: &Direction, grid: &Grid) {
    match dir {
        Direction::Up => {
            let mut i = grid.len() - 1;
            while i > 0 {
                if let Cell::Blizzards(_) = grid[i][pos.column] {
                    break;
                }
                i -= 1;
            }
            pos.row = i;
        },
        Direction::Right => {
            let mut j = 0;
            while j < grid[0].len() {
                if let Cell::Blizzards(_) = grid[pos.row][j] {
                    break;
                }
                j += 1;
            }
            pos.column = j;
        },
        Direction::Down => {
            let mut i = 0;
            while i < grid.len() {
                if let Cell::Blizzards(_) = grid[i][pos.column] {
                    break;
                }
                i += 1;
            }
            pos.row = i;
        },
        Direction::Left => {
            let mut j = grid[0].len() - 1;
            while j > 0 {
                if let Cell::Blizzards(_) = grid[pos.row][j] {
                    break;
                }
                j -= 1;
            }
            pos.column = j;
        }
    }
}

fn move_blizzards(g: &mut Grid, i: usize, j: usize, bs: &Vec<Direction>,
                  old_grid: &Grid) {
    for b in bs {
        let mut nb = step(&b, i, j);
        if let Cell::Wall = old_grid[nb.row][nb.column] {
            reenter(&mut nb, &b, &old_grid);
        }
        if let Cell::Blizzards(nbs) = &mut g[nb.row][nb.column] {
            nbs.push(*b);
        } else {
            assert!(false);
        }
    }
}

fn transfer_walls(old_grid: &Grid) -> Grid {
    let mut grid = vec![
        vec![Cell::Blizzards(Vec::<Direction>::new()); old_grid[0].len()]
        ; old_grid.len()];
    for (i, row) in old_grid.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            match el {
                Cell::Wall => { grid[i][j] = Cell::Wall; },
                _ => {}
            }
        }
    }
    grid
}

fn update_blizzards(old_grid: &Grid) -> Grid {
    let mut grid = transfer_walls(&old_grid);
    for (i, row) in old_grid.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if let Cell::Blizzards(bs) = el {
                move_blizzards(&mut grid, i, j, &bs, &old_grid);
            }
        }
    }
    grid
}

fn find_possible_moves(pos: &Position, grid: &Grid) -> Vec<Position> {
    static OFS: [[i32; 2]; 5] = [[0, 0], [-1, 0], [0, 1], [1, 0], [0, -1]];
    let mut r = Vec::<Position>::new();
    for f in OFS {
        if pos.row as i32 + f[0] < 0
           || pos.row as i32 + f[0] > (grid.len() - 1) as i32 {
            continue;
        }
        let row = (pos.row as i32 + f[0]) as usize;
        let column = (pos.column as i32 + f[1]) as usize;
        if let Cell::Blizzards(v) = &grid[row][column] {
            if v.len() == 0 {
                r.push(Position { row, column });
            }
        }
    }
    r
}

#[derive (Debug)]
struct State {
    time: u32,
    pos: Position
}

fn check_add(queue: &mut VecDeque::<State>, p: &Position, time: u32) {
    let mut found = false;
    for s in queue.iter() {
        if s.time < time {
            break;
        }
        if s.time == time && s.pos == *p {
            found = true;
            break;
        }
    }
    if !found {
        queue.push_front(State { pos: *p, time: time });
    }
}

fn search(grid: &Grid, start: &Position, end: &Position) -> (u32, Grid) {
    let mut queue = VecDeque::<State>::new();
    queue.push_front(State { time: 0, pos: *start });

    let mut grids = Vec::<Grid>::new();
    grids.push(grid.clone());

    let mut best_time = u32::MAX;
    while let Some(state) = queue.pop_back() {
        //scene::print(&grids[state.time as usize], &state.pos, state.time);
        if state.pos == *end {
            best_time = state.time;
            break;
        }
        if (state.time + 1) as usize == grids.len() {
            grids.push(update_blizzards(grids.last().unwrap()));
        }
        let new_grid = &grids.last().unwrap();
        let next_moves = find_possible_moves(&state.pos, &new_grid);
        for m in &next_moves {
            check_add(&mut queue, &m, state.time + 1);
        }
    }
    (best_time, grids[best_time as usize].clone())
}

fn problem_1(grid: &Grid, start: &Position, end: &Position) -> u32 {
    search(grid, start, end).0
}

fn problem_2(grid: &Grid, start: &Position, end: &Position) -> u32 {
    let (t1, grid1) = search(grid, start, end);
    let (t2, grid2) = search(&grid1, end, start);
    let (t3, _) = search(&grid2, start, end);
    t1 + t2 + t3
}

fn main() {
    let filename = util::parse_args();
    let grid = scene::parse(&filename);
    let start = scene::find_start_position(&grid);
    let exit = scene::find_exit_position(&grid);

    println!("Day 24, problem 1: {}", problem_1(&grid, &start, &exit));
    println!("Day 24, problem 2: {}", problem_2(&grid, &start, &exit));
}
