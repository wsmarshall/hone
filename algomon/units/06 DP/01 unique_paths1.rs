//part of the "grid DP" problem set

use std::error;
use std::io;

//takes "2D" approach
fn unique_paths(m: i32, n: i32) -> i32 {
    let rows: usize = m.try_into().unwrap();
    let cols: usize = n.try_into().unwrap();
    let mut grid = vec![vec![1; cols]; rows];
    for i in 1..rows {
        for j in 1..cols {
            grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
        }
    }
    grid[rows - 1][cols - 1]
}
