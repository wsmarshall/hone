use std::cmp;
use std::error;
use std::io;

fn lcs(word1: &Vec<char>, word2: &Vec<char>, dp: &mut Vec<Vec<i32>>) -> i32 {
    let m = word1.len();
    let n = word2.len();

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 || j == 0 {
                dp[i][j] = 0;
            } else if word1[i - 1] == word2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    dp[m][n]
}

fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let word1: Vec<char> = word1.chars().collect();
    let word2: Vec<char> = word2.chars().collect();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
    let lcs = lcs(&word1, &word2, &mut dp);

    let ans: i32;
    if m > n {
        ans = m as i32 - lcs;
    } else {
        ans = n as i32 - lcs;
    }

    ans
}
