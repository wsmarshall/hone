use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn knapsack_weight_only(weights: Vec<i32>) -> Vec<i32> {
    let m: usize = weights.len();
    let n: usize = weights.iter().sum::<i32>() as usize;
    let mut table = vec![vec![false; n + 1]; m + 1]; //sum validity

    table[0][0] = true;
    for w in 1..=m {
        for sum in 0..=n {}
    }
}
