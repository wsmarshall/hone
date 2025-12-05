use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn dfs(
    weights: &Vec<i32>,
    values: &Vec<i32>,
    n: usize,
    cap: i32,
    table: &mut Vec<Vec<i32>>,
) -> i32 {
    if n == 0 || cap <= 0 {
        return 0;
    }

    if table[n][cap as usize] != -1 {
        return table[n][cap as usize];
    }

    if weights[n - 1] > cap {
        return dfs(weights, values, n - 1, cap, table);
    } else {
        let with = dfs(weights, values, n - 1, cap - weights[n - 1], table) + values[n - 1];
        let without = dfs(weights, values, n - 1, cap, table);
        return cmp::max(with, without);
    }
}

fn knapsack(weights: Vec<i32>, values: Vec<i32>, max_weight: i32) -> i32 {
    let n = weights.len();
    let cap: usize = max_weight.try_into().unwrap();
    let mut table: Vec<Vec<i32>> = vec![vec![-1; cap + 1]; n + 1];

    dfs(&weights, &values, n, max_weight, &mut table)
}
