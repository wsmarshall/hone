use std::error;
use std::io;
use std::str::FromStr;

fn coin_game(coins: Vec<i32>, amount: i32) -> i32 {
    let m = coins.len();
    let n: usize = amount.try_into().unwrap();
    let mut table: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];

    table[0][0] = 1;

    for i in 1..=m {
        for j in 0..=n {
            table[i][j] += table[i - 1][j];
            if j as i32 >= coins[i - 1] {
                table[i][j] += table[i][j - coins[i - 1] as usize];
            }
        }
    }

    table[m][n]
}
