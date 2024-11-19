use std::{collections::VecDeque, fmt::Display};

use rand::{distributions::Uniform, prelude::*};

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Vec<Dir>>,
}

impl Graph {
    pub fn generate(count: usize, density: f32) -> Option<Self> {
        if !(0. ..=1.).contains(&density) {
            return None;
        }

        let edge_rng = Uniform::new(0f32, 1f32);
        let weight_rng = Uniform::new(0u32, 100_000u32);
        let mut rng = rand::thread_rng();

        let mut nodes = vec![vec![]; count];
        let mut weight = 0;
        let mut should_create_edge;

        for from in 0..count {
            for to in from + 1..count {
                if from == to {
                    continue;
                }

                should_create_edge = edge_rng.sample(&mut rng) < density;
                if !should_create_edge {
                    continue;
                }

                weight = weight_rng.sample(&mut rng);
                nodes[from].push(Dir::new(to as u32, weight));
                nodes[to].push(Dir::new(from as u32, weight));
            }
        }

        Some(Graph { nodes })
    }

    pub fn is_connected(&self) -> bool {
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut is_visited = vec![false; self.nodes.len()];
        let mut visited: usize = 0;

        queue.push_back(0);
        is_visited[0] = true;

        while let Some(current) = queue.pop_front() {
            visited += 1;
            for neighbour in &self.nodes[current] {
                if is_visited[neighbour.node as usize] {
                    continue;
                }

                is_visited[neighbour.node as usize] = true;
                queue.push_back(neighbour.node as usize);
            }
        }
        visited == self.nodes.len()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Dir {
    pub node: u32,
    pub weight: u32,
}

impl Dir {
    pub fn new(node: u32, weight: u32) -> Self {
        Self { node, weight }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[n: {}; w: {}]", self.node, self.weight)
    }
}
