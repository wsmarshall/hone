use std::cmp;
use std::error;
use std::io;

fn min(a: i32, b: i32, c: i32) -> i32 {
    cmp::min(a, cmp::min(b, c))
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
