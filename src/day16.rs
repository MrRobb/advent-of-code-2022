#![allow(clippy::must_use_candidate, clippy::missing_panics_doc, clippy::use_self)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use hashbrown::HashMap;
use itertools::{FoldWhile, Itertools};

#[derive(Debug, Clone)]
struct State {
    current: String,
    path: BTreeSet<String>,
    minutes: usize,
    pressure: usize,
}

impl State {
    fn not_visited(&self, node: &String) -> bool {
        !self.path.contains(node)
    }

    fn can_visit(&self, distances: &BTreeMap<(String, String), usize>, node: &str) -> bool {
        self.minutes > *distances.get(&(self.current.clone(), node.to_string())).unwrap()
    }

    fn visit(
        mut self,
        node: String,
        distances: &BTreeMap<(String, String), usize>,
        rates: &BTreeMap<String, usize>,
    ) -> State {
        self.minutes -= distances.get(&(self.current.clone(), node.clone())).unwrap() + 1;
        self.pressure += rates[&node] * self.minutes;
        self.path.insert(node.clone());
        self.current = node;
        self
    }
}

fn bfs_greedy(
    start: String,
    all_nodes: &[String],
    rates: &BTreeMap<String, usize>,
    distances: &BTreeMap<(String, String), usize>,
    initial_minutes: usize,
) -> usize {
    let state = State {
        current: start,
        path: BTreeSet::new(),
        minutes: initial_minutes,
        pressure: 0,
    };

    let mut q = VecDeque::new();
    q.push_back(state);

    let mut best_at_minute = vec![0; initial_minutes as usize + 1];
    let mut max_pressure = 0;

    while let Some(state) = q.pop_front() {
        if best_at_minute[state.minutes] > state.pressure {
            continue;
        }
        best_at_minute[state.minutes] = best_at_minute[state.minutes].max(state.pressure);

        for node in all_nodes {
            if rates[node] > 0 && state.not_visited(node) && state.can_visit(distances, node) {
                q.push_back(state.clone().visit(node.clone(), distances, rates));
            }
        }

        max_pressure = max_pressure.max(state.pressure);
    }

    max_pressure
}

fn build_all_paths(
    start: String,
    all_nodes: &[String],
    rates: &BTreeMap<String, usize>,
    distances: &BTreeMap<(String, String), usize>,
    initial_minutes: usize,
) -> HashMap<BTreeSet<String>, usize> {
    let state = State {
        current: start,
        path: BTreeSet::new(),
        minutes: initial_minutes,
        pressure: 0,
    };

    let mut q = VecDeque::new();
    q.push_back(state);

    // let mut visited = HashSet::new();
    let mut paths = HashMap::new();

    while let Some(state) = q.pop_front() {
        for node in all_nodes {
            if rates[node] > 0 && state.not_visited(node) && state.can_visit(distances, node) {
                q.push_back(state.clone().visit(node.clone(), distances, rates));
            }
        }
        *paths.entry(state.path.clone()).or_insert(0) =
            state.pressure.max(*paths.entry(state.path.clone()).or_insert(0));
    }

    paths
}

fn build_tunnels(input: &str) -> (BTreeMap<String, Vec<String>>, BTreeMap<String, usize>) {
    let mut tunnels = BTreeMap::new();
    let mut rates = BTreeMap::new();
    for line in input.lines() {
        let (valve, rest) = line
            .split_once("Valve ")
            .unwrap()
            .1
            .split_once(" has flow rate=")
            .unwrap();
        let (rate, leads_to) = rest
            .split_once("; tunnels lead to valves ")
            .unwrap_or_else(|| rest.split_once("; tunnel leads to valve ").unwrap());
        let rate = rate.parse::<usize>().unwrap();
        let leads_to = leads_to.split(", ").map(std::string::ToString::to_string).collect();

        rates.insert(valve.to_string(), rate);
        tunnels.insert(valve.to_string(), leads_to);
    }
    (tunnels, rates)
}

pub fn part1(input: &str) -> usize {
    let (tunnels, rates) = build_tunnels(input);
    // step(30, "AA".to_string(), tunnels, rates, BTreeSet::new())

    // Distances
    let mut distances = BTreeMap::new();
    for (node, adjs) in &tunnels {
        for adj in adjs {
            distances.insert((node.clone(), adj.clone()), 1);
        }
    }

    // Floyd-Warshall
    for k in tunnels.keys() {
        for i in tunnels.keys() {
            for j in tunnels.keys() {
                let ij = distances.get(&(i.clone(), j.clone()));
                let ik = distances.get(&(i.clone(), k.clone()));
                let kj = distances.get(&(k.clone(), j.clone()));
                let ikj = ik.zip(kj).map_or(usize::MAX, |(ik, kj)| ik + kj);
                let mind = ij.unwrap_or(&usize::MAX).min(&ikj);
                if *mind != usize::MAX {
                    distances.insert((i.clone(), j.clone()), *mind);
                }
            }
        }
    }

    // BFS
    bfs_greedy(
        "AA".to_string(),
        &tunnels.keys().cloned().collect::<Vec<_>>(),
        &rates,
        &distances,
        30,
    )
}

pub fn part2(input: &str) -> usize {
    let (tunnels, rates) = build_tunnels(input);

    // Distances
    let mut distances = BTreeMap::new();
    for (node, adjs) in &tunnels {
        for adj in adjs {
            distances.insert((node.clone(), adj.clone()), 1);
        }
    }

    // Floyd-Warshall
    for k in tunnels.keys() {
        for i in tunnels.keys() {
            for j in tunnels.keys() {
                let ij = distances.get(&(i.clone(), j.clone()));
                let ik = distances.get(&(i.clone(), k.clone()));
                let kj = distances.get(&(k.clone(), j.clone()));
                let ikj = ik.zip(kj).map_or(usize::MAX, |(ik, kj)| ik + kj);
                let mind = ij.unwrap_or(&usize::MAX).min(&ikj);
                if *mind != usize::MAX {
                    distances.insert((i.clone(), j.clone()), *mind);
                }
            }
        }
    }

    // Build all paths
    let paths = build_all_paths(
        "AA".to_string(),
        &tunnels.keys().cloned().collect::<Vec<_>>(),
        &rates,
        &distances,
        26,
    )
    .into_iter()
    .sorted_by_key(|path| path.1)
    .rev()
    .collect::<Vec<_>>();

    paths
        .iter()
        .enumerate()
        .map(|(i, human_path)| {
            paths[i + 1..]
                .iter()
                .fold_while(0, |max_pressure, elephant_path| {
                    if human_path.1 + elephant_path.1 <= max_pressure {
                        FoldWhile::Done(max_pressure)
                    } else if human_path.0.is_disjoint(&elephant_path.0) {
                        FoldWhile::Continue(max_pressure.max(human_path.1 + elephant_path.1))
                    } else {
                        FoldWhile::Continue(max_pressure)
                    }
                })
                .into_inner()
        })
        .max()
        .unwrap()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day16.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
