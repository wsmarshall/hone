use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn knapsack(weights: Vec<i32>, values: Vec<i32>, max_weight: i32) -> i32 {
    let n = weights.len(); //number of items
    let cap: usize = weights.iter().sum::<i32>().try_into().unwrap(); //total weight of all items

    let mut table: Vec<Vec<i32>> = vec![vec![0; cap + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=cap {
            if table[i][j] + weights[i - 1] < max_weight && j as i32 - weights[i - 1] >= 0 {
                table[i][j] = cmp::max(
                    table[i - 1][j],
                    table[i - 1][j - weights[i - 1] as usize] + values[i - 1],
                );
            } else {
                table[i][j] = table[i - 1][j];
            }
        }
    }

    table[n][cap]
}
