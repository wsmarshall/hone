use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut current: Vec<i32> = vec![0; n];
    let mut prev;

    let mut sum = 0;
    for i in 0..n {
        //fill in first row
        sum += grid[0][i];
        current[i] = sum;
    }
    prev = current.clone();

    for i in 1..m {
        current[0] = grid[i][0] + prev[0];
        for j in 1..n {
            current[j] = cmp::min(current[j - 1], prev[j]) + grid[i][j];
        }
        prev = current.clone();
    }

    prev[n - 1]
}
