use std::cmp;
use std::collections::{HashMap, HashSet};

use crate::valve::Valve;

pub fn search(valves: &Vec<Valve>,
              nonzero: &Vec<usize>,
              start_node: usize,
              dists: &Vec<Vec<u32>>) -> u32 {
    let mut max_score = 0;
    let mut cache = Cache::new();
    for node_id in nonzero {
        let time = dists[start_node][*node_id];
        let mut open_nodes = HashSet::<usize>::new();
        let new_score = dfs(valves, nonzero, dists, &mut open_nodes,
                            *node_id, time, 0, &mut cache);
        max_score = cmp::max(max_score, new_score);
    }
    max_score
}

#[derive(Eq, Hash, PartialEq)]
struct CacheKey {
    open_nodes: Vec<usize>,
    current_node: usize,
    time: u32
}

type Cache = HashMap<CacheKey, u32>;

fn create_cache_key(open_nodes_hash: &HashSet<usize>,
                    current_node: usize,
                    time: u32) -> CacheKey {
    let mut open_nodes: Vec<usize>
        = open_nodes_hash.clone().into_iter().collect();
    open_nodes.sort();
    CacheKey { open_nodes, current_node, time }
}

fn dfs(valves: &Vec<Valve>,
       nonzero: &Vec<usize>,
       dists: &Vec<Vec<u32>>,
       open_nodes: &mut HashSet<usize>,
       current: usize,
       mut time: u32,
       mut score: u32,
       mut cache: &mut Cache) -> u32 {
    // open node
    assert!(!open_nodes.contains(&current));
    assert!(time < 30);
    open_nodes.insert(current);
    time += 1;
    if time == 30 {
        return score;
    }
    assert!(time < 30);
    score += valves[current].rate * (30 - time);

    let mut best_score: u32 = score;
    for next in nonzero.iter().filter(|x| !open_nodes.contains(x)) {
        let steps = dists[current][*next];
        if time + steps >= 30 {
            continue;
        }
        let mut cloned_open_nodes = open_nodes.clone();
        let key = create_cache_key(&cloned_open_nodes, *next, time + steps);
        let child_score = match cache.get(&key) {
            Some(cached_score) => {
                score + cached_score
            },
            None => {
                let r = dfs(valves, nonzero, dists, &mut cloned_open_nodes,
                            *next, time + steps, score, &mut cache);
                cache.insert(key, r - score);
                r
            }
        };
        best_score = cmp::max(best_score, child_score);
    }
    best_score
}
