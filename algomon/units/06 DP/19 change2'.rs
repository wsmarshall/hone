use std::error;
use std::io;
use std::str::FromStr;

fn coin_game(coins: Vec<i32>, amount: i32) -> i32 {
    let m = coins.len();
    let n: usize = amount.try_into().unwrap();
    let mut current = vec![0; n + 1];

    let mut prev = current.clone();
    prev[0] = 1;

    for i in 1..=m {
        for j in 0..=n {
            current[j] += prev[j];
            if j as i32 >= coins[i - 1] {
                current[j] += current[j - coins[i - 1] as usize];
            }
        }
        prev = current;
        current = vec![0; n + 1];
    }

    prev[n]
}
