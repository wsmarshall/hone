use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn knapsack(weights: Vec<i32>, values: Vec<i32>, max_weight: i32) -> i32 {
    let n = weights.len(); //number of items
    let cap: usize = max_weight.try_into().unwrap();

    let mut table: Vec<Vec<i32>> = vec![vec![0; cap + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=cap {
            if (j as i32) < weights[i - 1] {
                table[i][j] = table[i - 1][j];
            } else {
                table[i][j] = cmp::max(
                    table[i - 1][j],
                    table[i - 1][j - weights[i - 1] as usize] + values[i - 1],
                );
            }
        }
    }

    table[n][cap]
}
