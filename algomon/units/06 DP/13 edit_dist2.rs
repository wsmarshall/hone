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

    for i in 1..=m {
        for j in 1..=n {
            if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = min(dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1]) + 1;
            }
        }
    }

    dp[m][n]
}
