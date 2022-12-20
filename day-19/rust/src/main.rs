use std::cmp;
use std::collections::VecDeque;

mod util;
mod blueprint;
use blueprint::Blueprint;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Robots {
    pub ore: u32,
    pub clay: u32,
    pub obsidian: u32,
    pub geode: u32
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Resources {
    pub ore: u32,
    pub clay: u32,
    pub obsidian: u32,
    pub geode: u32
}

#[derive(Debug, Clone)]
struct State {
    pub robots: Robots,
    pub resources: Resources,
    pub time: u32
}

fn create_child_state(state: & State) -> State {
    State {
        robots: state.robots.clone(),
        resources: Resources {
            ore: state.resources.ore + state.robots.ore,
            clay: state.resources.clay + state.robots.clay,
            obsidian: state.resources.obsidian + state.robots.obsidian,
            geode: state.resources.geode + state.robots.geode
        },
        time: state.time + 1
    }
}

fn create_start_state() -> State {
    let robots = Robots { ore: 1, clay: 0, obsidian: 0, geode: 0 };
    let resources = Resources { ore: 1, clay: 0, obsidian: 0, geode: 0 };
    State { robots, resources, time: 1 }
}

fn consider(q: &mut VecDeque<State>, s: State, bp: &Blueprint) {
    // Prune
    // it's meaningless to have more robots than the maximum bluprint
    // requirement for each resource
    if s.robots.ore > cmp::max(bp.geode.ore,
        cmp::max(bp.obsidian.ore, cmp::max(bp.clay, bp.ore)))
       || s.robots.clay > bp.obsidian.clay
       || s.robots.obsidian > bp.geode.obsidian {
        return;
    }
    // find duplicate/equivalent states
    let mut found = false;
    for t in q.iter().rev() {
        if t.robots.ore >= s.robots.ore
           && t.robots.clay >= s.robots.clay
           && t.robots.obsidian >= s.robots.obsidian
           && t.robots.geode >= s.robots.geode
           && t.resources.ore >= s.resources.ore
           && t.resources.clay >= s.resources.clay
           && t.resources.obsidian >= s.resources.obsidian
           && t.resources.geode >= s.resources.geode {
            found = true;
            break;
        }
        if t.time < s.time  {
            break;
        }
    }
    if !found {
        q.push_back(s);
    }
}

fn bfs(bp: &Blueprint, max_time: u32) -> u32 {
    let mut queue = VecDeque::<State>::new();
    queue.push_back(create_start_state());
    let mut max_geode: u32 = 0;
    while let Some(s) = queue.pop_front() {
        if s.time == max_time {
            max_geode = cmp::max(max_geode, s.resources.geode);
            continue;
        }
        if s.resources.ore >= bp.geode.ore
           && s.resources.obsidian >= bp.geode.obsidian {
            let mut cs = create_child_state(&s);
            cs.robots.geode += 1;
            cs.resources.ore -= bp.geode.ore;
            cs.resources.obsidian -= bp.geode.obsidian;
            consider(&mut queue, cs, &bp);
            continue;
        }
        if s.resources.ore >= bp.obsidian.ore
           && s.resources.clay >= bp.obsidian.clay {
            let mut cs = create_child_state(&s);
            cs.robots.obsidian += 1;
            cs.resources.ore -= bp.obsidian.ore;
            cs.resources.clay -= bp.obsidian.clay;
            consider(&mut queue, cs, &bp);
        }
        if s.resources.ore >= bp.ore {
            let mut cs = create_child_state(&s);
            cs.robots.ore += 1;
            cs.resources.ore -= bp.ore;
            consider(&mut queue, cs, &bp);
        }
        if s.resources.ore >= bp.clay {
            let mut cs = create_child_state(&s);
            cs.robots.clay += 1;
            cs.resources.ore -= bp.clay;
            consider(&mut queue, cs, &bp);
        }
        consider(&mut queue, create_child_state(&s), &bp);
    }
    max_geode
}

fn problem_1(blueprints: &Vec<Blueprint>) -> usize {
    let mut score: usize = 0;
    for (bpi, bp) in blueprints.iter().enumerate() {
        let max_geode = bfs(&bp, 24);
        score += (bpi + 1) * (max_geode as usize);
    }
    score
}

fn problem_2(blueprints: &Vec<Blueprint>) -> usize {
    let mut score: usize = 1;
    for bpi in 0..3 {
        let max_geode = bfs(&blueprints[bpi], 32);
        score *= max_geode as usize;
    }
    score
}

fn main() {
    let filename = util::parse_args();
    let blueprints: Vec<Blueprint> = util::create_reader(&filename)
        .map(|x| blueprint::parse(&x)).collect();

    println!("Day 19, problem 1: {}", problem_1(&blueprints));
    println!("Day 19, problem 2: {}", problem_2(&blueprints));
}
