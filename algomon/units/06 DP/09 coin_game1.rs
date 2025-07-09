use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn top_down(memo: &mut Vec<Vec<i32>>, prefix_sum: &Vec<i32>, l: usize, r: usize) -> i32 {
    if l == r {
        return prefix_sum[l];
    }

    if memo[l][r] != 0 {
        return memo[l][r];
    }

    let sum = prefix_sum[r] - prefix_sum[l - 1];

    let score = sum
        - cmp::min(
            top_down(memo, prefix_sum, l + 1, r),
            top_down(memo, prefix_sum, l, r - 1),
        );
    memo[l][r] = score;

    score
}

fn coin_game(coins: Vec<i32>) -> i32 {
    let n = coins.len();
    let mut prefix_sum: Vec<i32> = vec![0; n + 1];
    for i in 1..n {
        prefix_sum[i] = prefix_sum[i - 1] + coins[i - 1];
    }

    let mut memo: Vec<Vec<i32>> = vec![vec![0; n]; n];

    top_down(&mut memo, &prefix_sum, 1, n)
}
