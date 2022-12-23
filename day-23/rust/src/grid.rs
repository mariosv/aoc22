use std::fmt;

use crate::bbox::{Position, BBox, self};

#[derive (Debug, Clone)]
pub struct Grid {
    pub bbox: BBox,
    pub data: Vec::<Vec::<bool>>
}

impl Grid {
    pub fn new(elves: &Vec<Position>) -> Self {
        let bbox = bbox::create_bbox(&elves);
        let width: usize = (bbox.ur.column + 1 - bbox.ll.column)
            .try_into().unwrap();
        let height: usize = (bbox.ur.row + 1 - bbox.ll.row)
            .try_into().unwrap();
        let mut data = vec![vec![false; width]; height];
        for elf in elves {
            let p = Self::to_local(&bbox, &elf);
            data[p.row as usize][p.column as usize] = true;
        }
        Grid { bbox, data }
    }

    pub fn check_empty(&self, pos: &Position) -> bool {
        if pos.row < self.bbox.ll.row
           || pos.row > self.bbox.ur.row
           || pos.column < self.bbox.ll.column
           || pos.column > self.bbox.ur.column {
            return true;
        }
        let lp = Self::to_local(&self.bbox, &pos);

        !self.data[lp.row as usize][lp.column as usize]
    }

    pub fn empty_in_bbox_count(&self) -> usize {
        self.data.iter().map(|x| x.iter().filter(|y| !**y).count()).sum()
    }

    fn to_local(bbox: &BBox, p: &Position) -> Position {
        assert!(p.row - bbox.ll.row >= 0 && p.column - bbox.ll.column >= 0);
        Position {
            row: p.row - bbox.ll.row,
            column: p.column - bbox.ll.column
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "".to_string();
        for row in self.data.iter() {
            for el in row {
                if el == &true {
                    s += "#";
                } else {
                    s += ".";
                }
            }
            s += "\n";
        }
        write!(f, "b-box: {}\n{}\n", self.bbox, s)
    }
}
