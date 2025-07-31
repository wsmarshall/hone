use std::collections::HashSet;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn gen_sums(
    weights: &Vec<i32>,
    n: usize,
    sum: usize,
    sums: &mut HashSet<i32>,
    memo: &mut Vec<Vec<bool>>,
) {
    if memo[n][sum] {
        return;
    }
    if n == 0 {
        sums.insert(sum as i32);
        return;
    }

    gen_sums(weights, n - 1, sum, sums, memo);
    gen_sums(weights, n - 1, sum + weights[n - 1] as usize, sums, memo);
    memo[n][sum] = true;
}

fn knapsack_weight_only(weights: Vec<i32>) -> Vec<i32> {
    let mut sums: HashSet<i32> = HashSet::new();
    let m: usize = weights.len();
    let n: usize = weights.iter().sum::<i32>() as usize;
    let mut memo = vec![vec![false; n + 1]; m + 1]; //sums

    gen_sums(&weights, m, 0, &mut sums, &mut memo);

    let answer: Vec<i32> = sums.into_iter().collect::<Vec<i32>>();
    answer
}
