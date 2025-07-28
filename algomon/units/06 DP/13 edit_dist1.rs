use std::cmp;
use std::error;
use std::io;

fn min(a: i32, b: i32, c: i32) -> i32 {
    cmp::min(a, cmp::min(b, c))
}

fn dfs(m: usize, n: usize, word1: &String, word2: &String, memo: &mut Vec<Vec<i32>>) -> i32 {
    if m == 0 {
        return m;
    } else if n == 0 {
        return n;
    }
}

fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();

    let mut memo: Vec<Vec<i32>> = vec![vec![i32::MAX; n + 1]; m + 1];

    dfs(m, n, &word1, &word2, &mut memo)
}
