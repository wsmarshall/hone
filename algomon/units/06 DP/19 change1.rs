use std::error;
use std::io;
use std::str::FromStr;

fn dfs(coins: &Vec<i32>, table: &mut Vec<Vec<i32>>, amount: i32, start: usize, sum: i32) -> i32 {
    if sum == amount {
        return 1;
    } else if sum > amount || start > coins.len() {
        return 0;
    }

    let sum_usize: usize = sum.try_into().unwrap();

    if table[start][sum_usize] != -1 {
        return table[start][sum_usize];
    }

    let outcome = dfs(coins, table, amount, start + 1, sum + coins[start - 1])
        + dfs(coins, table, amount, start + 1, sum);

    table[start][sum_usize] = outcome;
    outcome
}

fn coin_game(coins: Vec<i32>, amount: i32) -> i32 {
    let m = coins.len();
    let n: usize = amount.try_into().unwrap();
    let mut table: Vec<Vec<i32>> = vec![vec![-1; n + 1]; m + 1];
    dfs(&coins, &mut table, amount, 1, 0);
    table[m][n]
}
