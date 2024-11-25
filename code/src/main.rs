#![allow(unused)]

use code::dijkstra;

const K: u32 = 10;
const G: usize = 10_000;

fn main() {
    let graph = code::graph::Graph::generate(G, 0.5);
    println!("done generating");

    // let standard = dijkstra::dijkstra_standard(&graph);
    let binary = dijkstra::dijkstra_binary(&graph);
    let fibonacci = dijkstra::dijkstra_fibonacci(&graph);

    // println!("std: {:?}", standard);
    // println!("bin: {:?}", binary);
    // println!("fib: {:?}", fibonacci);

    // for large G the results may not be equal,
    // but still be correct - the paths will differ,
    // however the minimum distances will not.
    //
    // println!("std==bin: {}", standard == binary);
    // println!("bin==fib: {}", binary == fibonacci);
    // println!("fib==std: {}", fibonacci == standard);
}
