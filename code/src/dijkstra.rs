use std::collections::VecDeque;

use crate::graph::*;

type BHeap = crate::binary::Heap<u32, usize>;
type FHeap = crate::fibonacci::Heap<u32, usize>;

#[derive(Debug, PartialEq, Eq)]
pub struct Result {
    pub distance: Vec<u32>,
    pub parent: Vec<u32>,
}

pub fn dijkstra_standard(graph: &Graph) -> Result {
    let n = graph.nodes.len();
    let mut distance = vec![u32::MAX; n];
    let mut parent = vec![u32::MAX; n];

    distance[0] = 0;
    parent[0] = 0;
    let mut q = (0..n).collect::<Vec<_>>();

    let mut current;
    let mut temp;
    let mut new_distance;
    let mut neighbour;

    for _ in 0..n {
        if q.is_empty() {
            break;
        }

        current = q[0];
        temp = 0;
        for (index, element) in q.iter().enumerate() {
            if distance[current] > distance[*element] {
                current = *element;
                temp = index;
            }
        }

        q.swap_remove(temp);

        for dir in &graph.nodes[current] {
            new_distance = distance[current] + dir.weight;
            neighbour = dir.node as usize;

            if new_distance < distance[neighbour] {
                parent[neighbour] = current as u32;
                distance[neighbour] = new_distance;
            }
        }
    }

    Result { distance, parent }
}

const PADDING: usize = 32;

pub fn dijkstra_binary(graph: &Graph) -> Result {
    let n = graph.nodes.len();
    let mut distance = vec![u32::MAX; n];
    let mut parent = vec![u32::MAX; n];
    let mut heap = BHeap::with_capacity(n + PADDING);

    distance[0] = 0;
    parent[0] = 0;
    heap.insert(0, 0);

    let mut new_distance;
    let mut neighbour;
    let mut current;

    while let Some(entry) = heap.extract_min() {
        current = entry.aux;
        if distance[current] < entry.key {
            continue;
        }

        for dir in &graph.nodes[current] {
            new_distance = distance[current] + dir.weight;
            neighbour = dir.node as usize;

            if new_distance < distance[neighbour] {
                parent[neighbour] = current as u32;
                distance[neighbour] = new_distance;
                heap.insert(new_distance, neighbour);
            }
        }
    }

    Result { distance, parent }
}

pub fn dijkstra_fibonacci(graph: &Graph) -> Result {
    let n = graph.nodes.len();
    let mut distance = vec![u32::MAX; n];
    let mut parent = vec![u32::MAX; n];
    let mut heap = FHeap::with_capacity(n + PADDING);

    for i in 0..n {
        heap.insert(u32::MAX, i);
    }

    distance[0] = 0;
    parent[0] = 0;
    heap.decrease_key(0, 0);

    let mut new_distance;
    let mut neighbour;
    let mut current;

    while let Some(entry) = heap.extract_min() {
        current = entry.aux;

        for dir in &graph.nodes[current] {
            new_distance = distance[current] + dir.weight;
            neighbour = dir.node as usize;

            if new_distance < distance[neighbour] {
                parent[neighbour] = current as u32;
                distance[neighbour] = new_distance;

                heap.decrease_key(neighbour, new_distance);
            }
        }
    }

    Result { distance, parent }
}

pub fn dijkstra_fibonacci_without_preload(graph: &Graph) -> Result {
    let n = graph.nodes.len();
    let mut distance = vec![u32::MAX; n];
    let mut parent = vec![u32::MAX; n];
    let mut heap_index = vec![n; n];
    let mut heap = FHeap::with_capacity(n + PADDING);

    distance[0] = 0;
    parent[0] = 0;
    heap_index[0] = heap.insert(0, 0);

    let mut new_distance;
    let mut neighbour;
    let mut current;

    while let Some(entry) = heap.extract_min() {
        current = entry.aux;

        for dir in &graph.nodes[current] {
            new_distance = distance[current] + dir.weight;
            neighbour = dir.node as usize;

            if new_distance >= distance[neighbour] {
                continue;
            }

            parent[neighbour] = current as u32;
            distance[neighbour] = new_distance;

            if heap_index[neighbour] == n {
                heap_index[neighbour] = heap.insert(new_distance, neighbour);
            } else {
                heap.decrease_key(heap_index[neighbour], new_distance);
            }
        }
    }

    Result { distance, parent }
}

pub fn recover_path(result: &Result, mut destination: u32) -> Vec<u32> {
    let mut path = vec![];
    while destination != 0 {
        path.push(destination);
        destination = result.parent[destination as usize];
    }
    path.push(0);
    path
}
