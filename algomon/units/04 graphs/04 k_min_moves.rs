use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::io;
use std::str::FromStr;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
    r: usize,
    c: usize,
}

fn build_Coordinate(r: i32, c: i32, val: i32) -> Coordinate {
    Coordinate { r: r, c: c }
}

fn get_neighbors(coord: Coordinate, grid: &Vec<Vec<i32>>) -> Vec<Coordinate> {
    let mut neighbors: Vec<Coordinate> = vec![];
    let deltaRow = [1, 2, 2, 1, -1, -2, -2, -1];
    let deltaCol = [-2, -1, 1, 2, 2, 1, -1, -2];
    for i in 0..deltaRow.len() {
        let neighborRow = coord.r as i32 + deltaRow[i];
        let neighborCol = coord.c as i32 + deltaCol[i];

        neighbors.push(build_Coordinate(neighborRow, neighborCol));
    }

    neighbors
}

fn bfs(x: i32, y: i32) -> i32 {}

fn get_knight_shortest_path(x: i32, y: i32) -> i32 {
    bfs(x, y)
}
