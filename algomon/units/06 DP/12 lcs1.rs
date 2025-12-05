use std::cmp;
use std::error;
use std::io;

//"top down" approach
fn lcs_dp(i: usize, j: usize, word1: &String, word2: &String, table: &mut Vec<Vec<i32>>) -> i32 {
    if i == 0 || j == 0 {
        //base case
        return 0;
    }

    if table[i][j] != -1 {
        return table[i][j];
    }

    let res: i32;
    if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
        res = lcs_dp(i - 1, j - 1, word1, word2, table) + 1;
    } else {
        res = cmp::max(
            lcs_dp(i - 1, j, word1, word2, table),
            lcs_dp(i, j - 1, word1, word2, table),
        );
    }
    table[i][j] = res;
    res
}

fn longest_common_subsequence(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let mut table: Vec<Vec<i32>> = vec![vec![-1; n + 1]; m + 1];

    lcs_dp(m, n, &word1, &word2, &mut table)
}
