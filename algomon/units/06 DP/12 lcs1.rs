use std::cmp;
use std::error;
use std::io;

fn lcs_dp(i: usize, j: usize, word1: &String, word2: &String, table: &mut Vec<Vec<i32>>) -> i32 {}

fn longest_common_subsequence(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let mut table: Vec<Vec<i32>> = vec![vec![-1; n + 1]; m + 1];

    lcs_dp(m, n, &word1, &word2, &mut table)
}
