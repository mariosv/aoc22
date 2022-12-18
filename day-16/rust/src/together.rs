use std::mem;
use std::cmp;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::valve::Valve;

pub fn search(valves: &Vec<Valve>,
              nonzero: &Vec<usize>,
              start_node: usize,
              dists: &Vec<Vec<u32>>) -> u32 {
    let mut max_score = 0;
    let mut cache = PairCache::new();
    // place the actors to active nodes, check all possible initial combinations
    for pair in nonzero.into_iter().combinations(2) {
        let a: usize = *pair[0];
        let b: usize = *pair[1];
        // Adding the 4min penalty and one minute to open the valve
        let time_a = dists[start_node][a] + 4 + 1;
        let time_b = dists[start_node][b] + 4 + 1;
        let mut open_nodes = HashSet::<usize>::new();
        open_nodes.insert(a);
        open_nodes.insert(b);
        let mut score = valves[a].rate * (30 - time_a);
        score += valves[b].rate * (30 - time_b);
        let new_score = pair_dfs(&valves, &nonzero, &dists, &mut open_nodes,
                                 a, b, time_a, time_b, score, &mut cache);
        max_score = cmp::max(max_score, new_score);
    }
    max_score
}

#[derive(Eq, Hash, PartialEq)]
pub struct PairCacheKey {
    open_nodes: Vec<usize>,
    node_a: usize,
    node_b: usize,
    time_a: u32,
    time_b: u32
}

pub type PairCache = HashMap<PairCacheKey, u32>;

pub fn create_pair_cache_key(open_nodes_hash: &HashSet<usize>,
                             node_a: usize,
                             node_b: usize,
                             time_a: u32,
                             time_b: u32) -> PairCacheKey {
    let mut open_nodes: Vec<usize>
        = open_nodes_hash.clone().into_iter().collect();
    open_nodes.sort();
    PairCacheKey { open_nodes, node_a, node_b, time_a, time_b }
}

// Advance both actors during a Depth-First search.
// Note that the actor that will open the next node sooner is selected and not
// necessarily the one that is behind in time.
// Extensive memoizing is applied
fn pair_dfs(valves: &Vec<Valve>,
            nonzero: &Vec<usize>,
            dists: &Vec<Vec<u32>>,
            open_nodes: &HashSet<usize>,
            mut current_a: usize,
            mut current_b: usize,
            mut time_a: u32,
            mut time_b: u32,
            score: u32,
            cache: &mut PairCache) -> u32 {
    // open node
    assert!(time_a <= 30 && time_b <= 30);
    let mut best_score: u32 = score;
    for next in nonzero.iter().filter(|x| !open_nodes.contains(x)) {
        let steps_a = dists[current_a][*next] + 1;
        let steps_b = dists[current_b][*next] + 1;
        let mut steps = steps_a;
        if (time_a + steps_a) > (time_b + steps_b) {
            // by convention we will always designate the next node to
            // actor A (current_a, time_a) and swap if necessary
            mem::swap(&mut time_a, &mut time_b);
            mem::swap(&mut current_a, &mut current_b);
            steps = steps_b;
        }
        if time_a + steps > 30 {
            continue;
        }
        let mut cloned_open_nodes = open_nodes.clone();
        cloned_open_nodes.insert(*next);
        let nscore = score + (30 - (time_a + steps)) * valves[*next].rate;
        // Check if a cached value exists
        let key = create_pair_cache_key(&cloned_open_nodes, *next, current_b,
                                        time_a + steps, time_b);
        let child_score = match cache.get(&key) {
            Some(cached_score) => nscore + cached_score,
            None => {
                let r = pair_dfs(valves, nonzero, dists,
                                 &mut cloned_open_nodes, *next, current_b,
                                 time_a + steps, time_b, nscore, cache);
                cache.insert(key, r - nscore);
                r
            }
        };
        best_score = cmp::max(best_score, child_score);
    }
    best_score
}
