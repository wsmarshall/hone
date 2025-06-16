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
        r: r as usize,
        c: c as usize,
        val: val,
    }
}

fn get_neighbors(coord: Coordinate, grid: &Vec<Vec<i32>>) -> Vec<Coordinate> {
    let mut neighbors: Vec<Coordinate> = vec![];
    let deltaRow = [-1, 0, 1, 0];
    let deltaCol = [0, 1, 0, -1];
    for i in 0..deltaRow.len() {
        let neighborRow = coord.r + deltaRow[i];
        let neighborCol = coord.c + deltaCol[i];
        if 0 <= neighborRow
            && neighborRow < image.len().try_into().unwrap()
            && 0 <= neighborCol
            && neighborCol < image[0].len().try_into().unwrap()
        {
            if grid[neighborRow as usize][neighborCol as usize] == 1 {
                neighbors.push(build_Coordinate(neighborRow, neighborCol));
            }
        }
    }

    neighbors
}

//start with 0,0 for initial coordinate
fn bfs(grid: &Vec<Vec<i32>>) -> i32 {
    let mut islands = 0;
    let mut queue = VecDeque::new();
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let height = grid.len();
    let width = grid[0].len(); //since it's a 2D matrix
    for i in 0..height {
        for j in 0..width {
            let current = build_coordinate(i, j, grid[i][j]);
            if visited.insert(current) {
                if current.val == 1 {
                    islands += 1;
                    queue.push_back(current.clone());
                    while !queue.is_empty() {
                        if let Some(coord) = queue.pop_front() {
                            let neighbors: Vec<Coordinate> = get_neighbors(coord, &grid);
                            for n in neighbors {
                                if visited.insert(n.clone()) {
                                    queue.push_back(n);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    islands
}

fn count_number_of_islands(grid: Vec<Vec<i32>>) -> i32 {
    bfs(&grid)
}
