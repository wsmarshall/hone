use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::io;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
    r: i32,
    c: i32,
    steps: i32,
}

fn build_Coordinate(r: i32, c: i32, steps: i32) -> Coordinate {
    Coordinate {
        r: r,
        c: c,
        steps: steps,
    }
}

fn get_neighbors(coord: &Coordinate) -> Vec<Coordinate> {
    let mut neighbors: Vec<Coordinate> = vec![];
    let deltaRow = [1, 2, 2, 1, -1, -2, -2, -1];
    let deltaCol = [-2, -1, 1, 2, 2, 1, -1, -2];
    for i in 0..deltaRow.len() {
        let neighborRow = coord.r as i32 + deltaRow[i];
        let neighborCol = coord.c as i32 + deltaCol[i];
        let distance = coord.steps + 1;
        neighbors.push(build_Coordinate(neighborRow, neighborCol, distance));
    }

    neighbors
}

fn bfs(x: i32, y: i32) -> i32 {
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut queue: VecDeque<Coordinate> = VecDeque::new();
    let mut steps: i32 = 0;

    queue.push_back(build_Coordinate(0, 0, 0));

    while !queue.is_empty() {
        if let Some(coord) = queue.pop_front() {
            if visited.insert(coord.clone()) {
                if coord.r == x && coord.c == y {
                    steps = coord.steps;
                    return steps;
                }

                let neighbors = get_neighbors(&coord);
                for i in neighbors {
                    queue.push_back(i);
                }
            }
        }
    }

    steps
}

fn get_knight_shortest_path(x: i32, y: i32) -> i32 {
    bfs(x, y)
}
