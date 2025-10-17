use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::io;
use std::str::FromStr;

fn shortest_path(graph: Vec<Vec<i32>>, a: i32, b: i32) -> i32 {
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(a);
    let mut visited: HashSet<i32> = HashSet::new();
    visited.insert(a);
    let mut distance = 0;

    while !queue.is_empty() {
        for i in 0..queue.len() {
            if let Some(current_node) = queue.pop_front() {
                if current_node == b {
                    return distance;
                }
                //the 'get neighbors' function
                let current_node_neighbors = &graph[current_node as usize];
                for j in 0..current_node_neighbors.len() {
                    if visited.insert(current_node_neighbors[j]) {
                        queue.push_back(current_node_neighbors[j]);
                    }
                }
            }
        }
        distance += 1;
    }
    distance
}
