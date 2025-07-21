use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];

    let mut sum = 0;
    for i in 0..n {
        //fill in first row
        sum += grid[0][i];
        dp[0][i] = sum;
    }

    sum = 0;
    for i in 0..m {
        //fill in first col
        sum += grid[i][0];
        dp[i][0] = sum;
    }
}
