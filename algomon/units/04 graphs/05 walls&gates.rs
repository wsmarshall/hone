use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

const INF: i32 = 2147483647;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
    r: usize,
    c: usize,
    steps: i32,
}

fn build_Coordinate(r: i32, c: i32, steps: i32) -> Coordinate {
    Coordinate {
        r: r as usize,
        c: c as usize,
        steps: steps,
    }
}

fn get_neighbors(coord: &Coordinate, grid: &Vec<Vec<i32>>) -> Vec<Coordinate> {
    let mut neighbors: Vec<Coordinate> = vec![];
    let deltaRow = [-1, 0, 1, 0];
    let deltaCol = [0, 1, 0, -1];
    for i in 0..deltaRow.len() {
        let neighborRow = coord.r as i32 + deltaRow[i];
        let neighborCol = coord.c as i32 + deltaCol[i];
        let distance = coord.steps + 1;

        if 0 <= neighborRow
            && neighborRow < grid.len().try_into().unwrap()
            && 0 <= neighborCol
            && neighborCol < grid[0].len().try_into().unwrap()
            && grid[neighborRow as usize][neighborCol as usize] == 2147483647
        {
            neighbors.push(build_Coordinate(neighborRow, neighborCol, distance));
        }
    }

    neighbors
}

fn bfs(map: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut distance_map = map.clone();
    let mut visited = HashSet::new();
    let mut queue: VecDeque<Coordinate> = VecDeque::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                queue.push_back(build_Coordinate(i as i32, j as i32, 0));
            }
        }
    }

    while !queue.is_empty() {
        if let Some(coord) = queue.pop_front() {
            if visited.insert((coord.r, coord.c)) {
                distance_map[coord.r][coord.c] = coord.steps;
                let neighbors = get_neighbors(&coord, &map);
                for i in neighbors {
                    queue.push_back(i);
                }
            }
        }
    }
    distance_map
}

fn map_gate_distances(dungeon_map: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    bfs(&dungeon_map)
}
