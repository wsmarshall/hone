use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn delete_string(costs: Vec<i32>, s1: String, s2: String) -> i32 {
    let m = s1.len();
    let n = s2.len();
    let str1: Vec<char> = s1.chars().collect();
    let str2: Vec<char> = s2.chars().collect();
    let mut table: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        table[i][0] = table[i - 1][0] + costs[(str1[i - 1] as u32 - 'a' as u32) as usize];
    }

    for i in 1..=n {
        table[0][i] = table[0][i - 1] + costs[(str2[i - 1] as u32 - 'a' as u32) as usize];
    }

    for i in 2..=m {
        for j in 2..=n {
            if str1[i - 1] == str2[j - 1] {
                table[i][j] = table[i - 1][j - 1];
            } else {
                table[i][j] = cmp::min(
                    table[i - 1][j] + costs[(str1[i - 1] as u32 - 'a' as u32) as usize],
                    table[i][j - 1] + costs[(str2[j - 1] as u32 - 'a' as u32) as usize],
                );
            }
        }
    }
    table[m][n]
}
