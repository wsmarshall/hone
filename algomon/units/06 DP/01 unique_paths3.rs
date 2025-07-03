use std::error;
use std::io;

fn unique_paths(m: i32, n: i32) -> i32 {
    let mut grid: Vec<Vec<i32>> = vec![vec![1; n as usize]; m as usize];

    let r = m as usize; //row
    let c = n as usize; //column

    for i in 1..r {
        for j in 1..c {
            grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
        }
    }

    grid[r - 1][c - 1]
}
