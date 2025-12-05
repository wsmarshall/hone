use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn dfs(level: usize, position: usize, triangle: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>) -> i32 {
    if level == triangle.len() {
        return 0;
    }

    if memo[level][position] != i32::MAX {
        return memo[level][position];
    }

    let left = dfs(level + 1, position, triangle, memo);
    let right = dfs(level + 1, position + 1, triangle, memo);
    let current = triangle[level][position];
    cmp::min(left, right) + current
}

fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let m = triangle.len();
    let n = triangle[m - 1].len();
    let mut memo = vec![vec![i32::MAX; n]; m];

    dfs(0, 0, &triangle, &mut memo)
}
