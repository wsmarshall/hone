use std::cmp;
use std::error;
use std::io;

fn min(a: i32, b: i32, c: i32) -> i32 {
    cmp::min(a, cmp::min(b, c))
}

fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();

    let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i as i32;
    }

    for i in 0..=n {
        dp[0][i] = i as i32;
    }
}
