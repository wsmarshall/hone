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
    if a == b {
        return distance;
    }

    while !queue.is_empty() {
        distance += 1;
        for i in 0..queue.len() {
            if let Some(current_node) = queue.pop_front() {
                let current_node_neighbors = &graph[current_node as usize];
                for j in 0..current_node_neighbors.len() {
                    if current_node_neighbors[i] == b {
                        return distance;
                    } else if visited.insert(current_node_neighbors[i]) {
                        queue.push_back(current_node_neighbors[i]);
                    } else {
                        continue;
                    }
                }
            }
        }
    }
    distance
}
