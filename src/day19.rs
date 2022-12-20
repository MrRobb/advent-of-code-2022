#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::nonminimal_bool,
    clippy::use_self,
    clippy::cognitive_complexity
)]

use std::collections::VecDeque;
use std::ops::{Index, IndexMut};
use std::sync::Arc;

use enum_iterator::Sequence;
use hashbrown::HashSet;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::prelude::*;
use scan_fmt::scan_fmt;

#[derive(Sequence, Clone, Copy)]
enum Material {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

impl<T> Index<Material> for [T; 4] {
    type Output = T;

    fn index(&self, index: Material) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Material> for [T; 4] {
    fn index_mut(&mut self, index: Material) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Blueprint {
    id: usize,
    costs: [[usize; 4]; 4],
}

impl Blueprint {
    fn parse(input: &str) -> Self {
        let (
            i_blueprint,
            cost_ore,
            cost_clay,
            cost_obsidian_ore,
            cost_obsidian_clay,
            cost_geode_ore,
            cost_geode_obsidian,
        ) = scan_fmt!(
            input,
            "Blueprint {d}: Each ore robot costs {d} ore. Each clay robot costs {d} ore. Each obsidian robot costs \
             {d} ore and {d} clay. Each geode robot costs {d} ore and {d} obsidian.",
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize
        )
        .unwrap();
        Self {
            id: i_blueprint,
            costs: [
                [cost_ore, 0, 0, 0],
                [cost_clay, 0, 0, 0],
                [cost_obsidian_ore, cost_obsidian_clay, 0, 0],
                [cost_geode_ore, 0, cost_geode_obsidian, 0],
            ],
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    blueprint: Arc<Blueprint>,
    minutes: usize,
    robots: [usize; 4],
    materials: [usize; 4],
}

impl State {
    fn can_build(&self, material: Material) -> bool {
        self.blueprint.costs[material]
            .iter()
            .zip(self.materials.iter())
            .all(|(cost, material)| material >= cost)
    }

    fn needs_robot(&self, robot_material: Material, needed_by: Material) -> bool {
        self.robots[robot_material] < self.blueprint.costs[needed_by][robot_material]
    }

    fn any_needs_robot(&self, robot_material: Material) -> bool {
        enum_iterator::all::<Material>().any(|material| self.needs_robot(robot_material, material))
    }

    const fn extract_materials(mut self) -> State {
        self.materials = [
            self.materials[0] + self.robots[0],
            self.materials[1] + self.robots[1],
            self.materials[2] + self.robots[2],
            self.materials[3] + self.robots[3],
        ];
        self
    }

    fn build_robot(mut self, material: Material) -> State {
        self.materials = [
            self.materials[0] - self.blueprint.costs[material][0],
            self.materials[1] - self.blueprint.costs[material][1],
            self.materials[2] - self.blueprint.costs[material][2],
            self.materials[3] - self.blueprint.costs[material][3],
        ];
        self.robots[material] += 1;
        self
    }

    const fn tick(mut self) -> State {
        self.minutes -= 1;
        self
    }
}

fn max_geodes(blueprint: Blueprint, minutes: usize) -> usize {
    // Create start node
    let start = State {
        blueprint: Arc::new(blueprint),
        minutes,
        robots: [1, 0, 0, 0],
        materials: [0, 0, 0, 0],
    };

    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([start]);

    let mut max_geodes = 0;

    while let Some(state) = queue.pop_front() {
        // println!("{:?}", (state.materials, state.robots));
        if state.minutes == 0 {
            if state.materials[3] > max_geodes {
                max_geodes = state.materials[3];
            }
            continue;
        }

        if visited.contains(&(state.materials, state.robots)) {
            continue;
        }
        visited.insert((state.materials, state.robots));

        let new_state = state.clone().extract_materials().tick();

        // If possible, always build geode robots
        if state.can_build(Material::Geode) {
            queue.push_back(new_state.clone().build_robot(Material::Geode));
        }

        // If we need more obsidian robots, build obsidian robot
        if state.needs_robot(Material::Obsidian, Material::Geode) && state.can_build(Material::Obsidian) {
            queue.push_back(new_state.clone().build_robot(Material::Obsidian));
        }

        // If any material needs more ore robots, build ore robot
        if state.any_needs_robot(Material::Ore) && state.can_build(Material::Ore) {
            queue.push_back(new_state.clone().build_robot(Material::Ore));
        }

        // If we need more clay robots, build clay robot
        if state.needs_robot(Material::Clay, Material::Obsidian) && state.can_build(Material::Clay) {
            queue.push_back(new_state.clone().build_robot(Material::Clay));
        }

        queue.push_back(new_state);
    }

    max_geodes
}

pub fn quality_levels(input: &str) -> usize {
    input.lines()
        .par_bridge()
        .map(Blueprint::parse)
        .map(|b| (b.id, max_geodes(b, 24)))
        .progress_count(30)
        // Get quality level
        .map(|(id, max_geodes)| id * max_geodes)
        .sum()
}

pub fn open_geodes(input: &str) -> usize {
    input
        .lines()
        .map(Blueprint::parse)
        .take(3)
        .map(|b| max_geodes(b, 32))
        .progress_count(3)
        .product()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day19.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", quality_levels(&input));
    println!("PART 2 = {}", open_geodes(&input));
    println!("Execution time: {:?}", now.elapsed());
}
