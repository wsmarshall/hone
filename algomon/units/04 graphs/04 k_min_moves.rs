use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::io;
use std::str::FromStr;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
    r: usize,
    c: usize,
    val: i32,
}

fn build_Coordinate(r: i32, c: i32, val: i32) -> Coordinate {
    Coordinate {
        r: r,
        c: c,
        val: val,
    }
}

fn get_neighbors(coord: Coordinate, grid: &Vec<Vec<i32>>) -> Vec<Coordinate> {
    let mut neighbors: Vec<Coordinate> = vec![];
    let deltaRow = [-1, 0, 1, 0];
    let deltaCol = [0, 1, 0, -1];
    for i in 0..deltaRow.len() {
        let neighborRow = coord.r as i32 + deltaRow[i];
        let neighborCol = coord.c as i32 + deltaCol[i];
        if 0 <= neighborRow
            && neighborRow < grid.len().try_into().unwrap()
            && 0 <= neighborCol
            && neighborCol < grid[0].len().try_into().unwrap()
        {
            if grid[neighborRow as usize][neighborCol as usize] == 1 {
                neighbors.push(build_Coordinate(neighborRow, neighborCol, 1));
            }
        }
    }

    neighbors
}

fn get_knight_shortest_path(x: i32, y: i32) -> i32 {
    // WRITE YOUR BRILLIANT CODE HERE
    0
}
