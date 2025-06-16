use std::error;
use std::io;
use std::str::FromStr;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
    r: usize,
    c: usize,
}

fn build_Coordinate(r: i32, c: i32) -> Coordinate {
    Coordinate {
        r: r as usize,
        c: c as usize,
    }
}

fn get_neighbors(r: i32, c: i32, color: i32, grid: &Vec<Vec<i32>>) -> Vec<Coordinate> {
    let mut neighbors: Vec<Coordinate> = vec![];
    let deltaRow = [-1, 0, 1, 0];
    let deltaCol = [0, 1, 0, -1];
    for i in 0..deltaRow.len() {
        let neighborRow = r + deltaRow[i];
        let neighborCol = c + deltaCol[i];
        if 0 <= neighborRow
            && neighborRow < image.len().try_into().unwrap()
            && 0 <= neighborCol
            && neighborCol < image[0].len().try_into().unwrap()
        {
            if grid[neighborRow as usize][neighborCol as usize] == color {
                neighbors.push(build_Coordinate(neighborRow, neighborCol));
            }
        }
    }

    neighbors
}

//starts at 0, 0 for the first coordinate
fn count_number_of_islands(grid: Vec<Vec<i32>>) -> i32 {
    // WRITE YOUR BRILLIANT CODE HERE
    0
}
