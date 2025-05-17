use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::io;
use std::str::FromStr;

fn shortest_path(graph: Vec<Vec<i32>>, a: i32, b: i32) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back(graph[a as usize]);
    let mut visited = HashSet::new();

    let mut distance = 0;

    while !queue.is_empty() {
        if let Some(current) = queue.pop_front() {
            for i in 0..current.len() {
                if visited.insert(current) {
                    queue.push_back(graph[i]);
                } else {
                    continue;
                }
            }
        }
    }

    distance
}
