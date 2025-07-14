use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn coin_game(coins: Vec<i32>) -> i32 {
    let n = coins.len();
    let mut prefix_sum = vec![0; n + 1];
    for i in 1..n + 1 {
        prefix_sum[i] = prefix_sum[i - 1] + coins[i - 1];
    }

    let mut game_matrix = vec![vec![0; n + 1]; n + 1];
    for size in 0..n {
        let mut l = 1;
        while l + size <= n {
            let r = l + size;
            let sum = prefix_sum[r] - prefix_sum[l - 1];
            if l == r {
                //one coin
                game_matrix[l][r] = sum;
            } else {
                //check previous results to minimize opponent's score
                game_matrix[l][r] = sum - cmp::min(game_matrix[l + 1][r], game_matrix[l][r - 1]);
            }
            l += 1;
        }
    }

    game_matrix[1][n]
}
