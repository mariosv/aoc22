use std::process;
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct Valve {
    pub name: String,
    pub rate: u32,
    pub connections: Vec<usize>
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

fn create_ids_from_names(filename: &String) -> HashMap<String, usize> {
    let lines = create_reader(&filename);
    let names: Vec<String> = lines
        .map(|x| x.split(' ').nth(1).unwrap().to_string())
        .collect();
    let mut map = HashMap::<String, usize>::new();
    for  (i, name) in names.iter().enumerate() {
        map.insert(name.to_string(), i);
    }
    return map;
}

fn parse_valve(s: &String, names_to_ids: &HashMap<String, usize>) -> Valve {
    let mut p = s.trim().split(' ');

    // name
    let name: String = p.nth(1).unwrap().to_string();

    // rate
    let rate_str = p.nth(2).unwrap().split('=').nth(1).unwrap().to_string();
    // remove ';'
    let qi = rate_str.find(';').unwrap();
    let rate: u32 = rate_str[..qi].parse().unwrap();

    // connections
    let mut connections = Vec::<usize>::new();
    for _ in 0..4 { p.next(); }
    while let Some(n) = p.next() {
        let comma = n.find(',');
        let t: String = match comma {
            Some(x) => n[..x].to_string(),
            None => n.to_string()
        };
        connections.push(names_to_ids[&t]);
    }

    Valve { name, rate, connections }
}

pub fn parse(filename: &String) -> (Vec<Valve>, usize) {
    // first pass, create ids
    let names_to_ids = create_ids_from_names(&filename);
    // second pass
    (create_reader(&filename)
        .map(|x| parse_valve(&x, &names_to_ids)).collect(),
     names_to_ids["AA"])
}
