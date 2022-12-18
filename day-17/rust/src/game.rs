use std::cmp;

#[derive(Debug)]
pub enum Move {
    Left,
    Right
}

fn define_rock_types() -> Vec<Vec<Vec<i32>>> {
    let row = vec!(vec![1; 4]; 1);

    let mut cross = vec!(vec![1; 3]; 3);
    cross[0][0] = 0;
    cross[2][0] = 0;
    cross[0][2] = 0;
    cross[2][2] = 0;

    let mut angle = vec!(vec![1; 3]; 3);
    angle[1][0] = 0;
    angle[1][1] = 0;
    angle[2][0] = 0;
    angle[2][1] = 0;

    let column = vec!(vec![1; 1]; 4);

    let boxi = vec!(vec![1; 2]; 2);

    let mut all = Vec::<Vec::<Vec::<i32>>>::new();
    all.push(row);
    all.push(cross);
    all.push(angle);
    all.push(column);
    all.push(boxi);

    all
}

type Rock = Vec<Vec<i32>>;

#[derive(Debug, PartialEq, Eq)]
struct Status {
    rock_type_index: usize,
    move_index: usize,
    offsets: [u32; 7]
}

#[derive(Debug)]
pub struct Game {
    grid: Vec<Vec<i32>>,
    rock: Option<[i32; 2]>,
    top: i32,
    rock_types: Vec<Rock>,
    moves: Vec<Move>,
    rock_type_index: usize,
    move_index: usize,
    status: Vec<Status>,
    prev_tops: Vec<i32>
}

impl Game {
    pub fn new(moves: Vec<Move>) -> Self {
        let grid = vec!(vec![0; 7]; 1000000);
        let rocks = define_rock_types();
        let rock_count = rocks.len() - 1;
        let move_index = moves.len() - 1;

        Game {
            grid: grid,
            rock: None,
            top: -1,
            rock_types: rocks,
            moves: moves,
            rock_type_index: rock_count,
            move_index: move_index,
            status: Vec::new(),
            prev_tops: Vec::new()
        }
    }

    fn create_status(&self) -> Status {
        let mut offsets = [0 as u32; 7];
        for j in 0..7 {
            let mut i = self.top;
            loop {
                if self.grid[i as usize][j as usize] != 1 {
                    i -= 1;
                    if i == -1 {
                        break;
                    }
                } else {
                    break;
                }
            }
            offsets[j] = (self.top - i) as u32;
        }
        Status {
            rock_type_index: self.rock_type_index,
            move_index: self.move_index,
            offsets: offsets
        }
    }

    pub fn enter_next(&mut self) {
        self.rock_type_index += 1;
        if self.rock_type_index == self.rock_types.len() {
            self.rock_type_index = 0;
        }
        let position: [i32; 2] = [self.top + 4, 2];
        assert!(!self.rock.is_some());
        self.rock = Some(position);
    }

    pub fn next_move(&mut self) -> bool {
        self.move_index += 1;
        if self.move_index == self.moves.len() {
            self.move_index = 0;
        }
        assert!(self.move_index < self.moves.len());
        assert!(self.rock.is_some());
        let current_pos = self.rock.unwrap();
        let mut new_pos = current_pos;
        new_pos[1] = match self.moves[self.move_index] {
            Move::Left => current_pos[1] - 1,
            Move::Right => current_pos[1] + 1
        };
        if !self.collides(&new_pos) {
            self.rock = Some(new_pos);
            return true;
        }
        return false;
    }

    pub fn fall(&mut self) -> bool {
        assert!(self.rock.is_some());
        let current_pos = self.rock.unwrap();
        let mut new_pos = current_pos;
        new_pos[0] -= 1;
        if !self.collides(&new_pos) {
            self.rock = Some(new_pos);
            return true;
        }
        return false;
    }

    fn collides(&self, pos: &[i32; 2]) -> bool {
        let rock_type = &self.rock_types[self.rock_type_index];
        for i in 0..rock_type.len() {
            let row = &rock_type.iter().nth(i).unwrap();
            for j in 0..row.len() {
                if row.iter().nth(j).unwrap() == &1 {
                    let ai = i as i32 + pos[0];
                    let aj = j as i32 + pos[1];
                    if ai < 0 ||  aj < 0 || aj > 6 {
                        return true;
                    }
                    if self.grid[ai as usize][aj as usize] == 1 {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn land(&mut self) {
        assert!(self.rock.is_some());
        assert!(!self.collides(&self.rock.unwrap()));
        let rock_type = &self.rock_types[self.rock_type_index];
        let pos = self.rock.unwrap();
        for i in 0..rock_type.len() {
            let row = &rock_type.iter().nth(i).unwrap();
            for j in 0..row.len() {
                if row.iter().nth(j).unwrap() == &1 {
                    let ai = i as i32 + pos[0];
                    let aj = j as i32 + pos[1];
                    assert!(ai >= 0 && aj >= 0 && aj <= 6);
                    self.grid[ai as usize][aj as usize] = 1;
                    self.top = cmp::max(self.top, ai);
                }
            }
        }
        self.rock = None;
        self.status.push(self.create_status());
        self.prev_tops.push(self.top);
    }
}

fn find_previous_same_status(status: &Vec<Status>) -> Option<usize> {
    for i in 0..(status.len() - 1) {
        if status[status.len() - 1] == status[i] {
            return Some(i);
        }
    }
    None
}

pub fn simulate(moves: Vec<Move>, rounds: usize) -> usize {
    let mut game = Game::new(moves);

    let mut r: usize = 0;
    loop {
        game.enter_next();
        loop {
            game.next_move();
            if !game.fall() {
                game.land();
                let he = game.top as usize;
                if let Some(s) = find_previous_same_status(&game.status) {
                    let hs = game.prev_tops[s] as usize;
                    let n = (rounds - s) / (r - s);
                    let x = rounds - s - n * (r - s);
                    assert!(s + x < r);
                    let hx = game.prev_tops[s + x] as usize - hs;
                    return hs  + n * (he - hs) + hx;
                }
                break;
            }
        }
        r += 1;
        if r == (rounds + 1) {
            break;
        }
    }
    game.top as usize + 1
}
