use std::error;
use std::io;

fn lcs(m: usize, n: usize, word1: &Vec<char>, word2: &Vec<char>, dp: &mut Vec<Vec<i32>>) -> i32 {
    for 
}

fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let word1: Vec<char> = word1.chars().collect();
    let word2: Vec<char> = word2.chars().collect();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];
    let lcs = lcs(0, 0, &word1, &word2, &mut dp);
    
    let ans: i32;
    if m > n {
        ans = m as i32 - lcs;
    } else {
        ans = n as i32 - lcs;
    }
    
    ans
}