use std::cmp;
use std::error;
use std::io;

fn min(a: i32, b: i32, c: i32) -> i32 {
    cmp::min(a, cmp::min(b, c))
}

fn dfs(m: usize, n: usize, word1: &String, word2: &String, memo: &mut Vec<Vec<i32>>) -> i32 {
    if m == 0 {
        return n as i32;
    } else if n == 0 {
        return m as i32;
    }

    if memo[m][n] != i32::MAX {
        return memo[m][n];
    }

    let mut cost;
    if word1.as_bytes()[m - 1] == word2.as_bytes()[n - 1] {
        cost = 0;
    } else {
        cost = 1;
    }

    memo[m][n] = min(
        dfs(m - 1, n - 1, word1, word2, memo) + cost,
        dfs(m - 1, n, word1, word2, memo) + 1,
        dfs(m, n - 1, word1, word2, memo) + 1,
    );

    memo[m][n]
}

fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();

    let mut memo: Vec<Vec<i32>> = vec![vec![i32::MAX; n + 1]; m + 1];

    dfs(m, n, &word1, &word2, &mut memo)
}
