/**
 * version that remained -- no recollection of attempting
 */
use std::error;
use std::io;
use std::str::FromStr;

fn dfs(denoms: &Vec<i32>, target: i32, sum: i32, start_index: usize, memo: &mut Vec<i32>) -> i32 {
    if sum == target {
        return 0;
    }

    if sum > target {
        return std::i32::MAX - 9;
    }

    if memo[sum as usize] != -1 {
        return memo[sum as usize];
    }

    let mut inter_result = std::i32::MAX - 9;
    for i in start_index..denoms.len() {
        let mut temp;
        temp = dfs(denoms, target, sum + denoms[i], i, memo);
        inter_result = std::cmp::min(inter_result, temp + 1);
    }
    memo[sum as usize] = inter_result;
    inter_result
}

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut memo = vec![-1; (amount + 1).try_into().unwrap()];
    let result = dfs(&coins, amount, 0, 0, &mut memo);
    if result == std::i32::MAX - 9 {
        return -1;
    }
    result
}
