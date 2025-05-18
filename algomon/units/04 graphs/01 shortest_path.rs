use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::io;
use std::str::FromStr;

fn shortest_path(graph: Vec<Vec<i32>>, a: i32, b: i32) -> i32 {
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(a);
    let mut visited: HashSet<i32> = HashSet::new();

    let mut distance = 0;

    while !queue.is_empty() {
        distance += 1;
        if let Some(num) = queue.pop_front() {
            let current_node_neighbors = &graph[num as usize];
            for i in 0..current_node_neighbors.len() {
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
    distance
}
