use std::cmp;
use std::error;
use std::io;

fn longest_common_subsequence(word1: String, word2: String) -> i32 {
    let m: usize = word1.len();
    let n: usize = word2.len();

    let mut table: Vec<Vec<i32>> = vec![vec![-1; n + 1]; m + 1];
    for i in 0..=m {
        for j in 0..=n {
            if i == 0 || j == 0 {
                table[i][j] = 0;
            } else if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
                table[i][j] = table[i - 1][j - 1] + 1;
            } else {
                table[i][j] = cmp::max(table[i - 1][j], table[i][j - 1]);
            }
        }
    }
    //println!("table: {:?}", table);
    table[m][n]
}
