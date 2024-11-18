use rand::{distributions::Uniform, prelude::*};
use savefile::prelude::*;
use savefile_derive::Savefile;

#[derive(Savefile)]
#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Vec<Dir>>,
}

impl Graph {
    fn generate(count: usize, density: f32) -> Option<Self> {
        if !(0. ..1.).contains(&density) {
            return None;
        }

        let edge_rng = Uniform::new(0f32, 1f32);
        let position_rng = Uniform::new(0i32, 1_000_000i32);
        let mut rng = rand::thread_rng();

        let positions = (0..count)
            .map(|_| (position_rng.sample(&mut rng), position_rng.sample(&mut rng)))
            .collect::<Vec<_>>();

        let mut should_create_edge = move || edge_rng.sample(&mut rng) < density;
        fn square_magnitude(a: (i32, i32), b: (i32, i32)) -> usize {
            ((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)) as usize
        }

        let mut nodes = vec![vec![]; count];
        let mut weight = 0;

        for from in 0..count {
            for to in from + 1..count {
                if from == to {
                    continue;
                }
                if !should_create_edge() {
                    continue;
                }

                weight = square_magnitude(positions[from], positions[to]);
                nodes[from].push(Dir::new(to, weight));
                nodes[to].push(Dir::new(from, weight));
            }
        }

        Some(Graph { nodes })
    }
}

#[derive(Clone, Copy, Savefile)]
#[derive(Debug)]
pub struct Dir {
    node: usize,
    weight: usize,
}

impl Dir {
    pub fn new(node: usize, weight: usize) -> Self {
        Self { node, weight }
    }
}
