use std::fmt;
use std::cmp;

#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    pub row: i32,
    pub column: i32
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

#[derive (Debug, Clone)]
pub struct BBox {
    pub ll: Position,
    pub ur: Position
}

pub fn create_bbox(data: &Vec<Position>) -> BBox {
    let mut bbox = BBox {
        ll: Position { row: 1e8 as i32, column: 1e8 as i32 },
        ur: Position { row: -1e8 as i32, column: -1e8 as i32}
    };
    for t in data {
        bbox.ll.row = cmp::min(bbox.ll.row, t.row);
        bbox.ll.column = cmp::min(bbox.ll.column, t.column);
        bbox.ur.row = cmp::max(bbox.ur.row, t.row);
        bbox.ur.column = cmp::max(bbox.ur.column, t.column);
    }
    bbox
}

impl fmt::Display for BBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} -> {})", self.ll, self.ur)
    }
}
