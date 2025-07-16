use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;
use std::collections::HashSet;

fn tddp(n: usize, weights: &Vec<i32>, set: &mut HashSet<i32>, memo: &mut Vec<Vec<bool>>) -> Vec<i32> {
    if n == 0 {
        return Vec::from_iter(set.iter().cloned());
    }
    if memo()
    vec![]
}

fn knapsack_weight_only(weights: Vec<i32>) -> Vec<i32> {
    let mut set: HashSet<i32> = HashSet::new();
    let m: usize = weights.len();
    let n: usize = weights.iter().sum::<i32>() as usize;
    let mut memo = vec![vec![false; n]; m];

    tddp(m, &weights, &mut set, &mut memo)
}