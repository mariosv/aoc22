use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: VecDeque<u64>,
    pub divisible_by: u64,
    pub activity: u32,
    operation: Operation,
    target_1: u32,
    target_2: u32
}

impl Monkey {
    pub fn parse(lines: &mut impl Iterator<Item = String>) -> Monkey {
        lines.next().unwrap();
        // parse items
        let items = lines.next().unwrap().split(":").nth(1).unwrap().split(", ")
            .map(|x| x.trim().parse::<u64>()
                 .unwrap()).collect::<VecDeque<u64>>();
        // parse operation
        let operation = Operation::parse(&lines.next().unwrap());
        // parse divisible
        let divisible_by = lines.next().unwrap().split("by ").nth(1).unwrap()
            .parse::<u64>().unwrap();
        // parse target
        let target_1 = Self::parse_target(&lines.next().unwrap());
        let target_2 = Self::parse_target(&lines.next().unwrap());

        Monkey { items, operation, divisible_by, activity: 0,
            target_1, target_2 }
    }

    pub fn inspect(&self, v: u64) -> u64 {
        self.operation.apply(v)
    }

    pub fn forward(&self, v: u64) -> usize {
        if v % self.divisible_by == 0 {
            return self.target_1 as usize;
        }
        self.target_2 as usize
    }

    fn parse_target(s: &String) -> u32 {
        s.split("monkey ").nth(1).unwrap().parse().unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum Operator { Add, Multiply }

#[derive(Debug, Clone)]
pub struct Operation {
    pub operator: Operator,
    pub constant: Option<u64>
}

impl Operation {
    pub fn apply(&self, v: u64) -> u64 {
        let operand = match self.constant {
            Some(x) => x,
            None => v
        };
        match self.operator {
            Operator::Add => v + operand,
            Operator::Multiply => v * operand
        }
    }

    pub fn parse(line: &String) -> Operation {
        let mut s = line.split("=").nth(1).unwrap().split_whitespace();
        let op_str = s.nth(1).unwrap();
        let operator: Operator = match op_str {
            "*" => Operator::Multiply ,
            "+" => Operator::Add,
            _ => panic!("I don't know this op")
        };
        let constant_str = s.next().unwrap();
        let constant = match constant_str {
            "old" => None,
            _ => Some(constant_str.parse::<u64>().unwrap())
        };
        Operation { operator, constant }
    }
}
