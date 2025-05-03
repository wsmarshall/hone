fn dfs(coins: &Vec<i32>, amount: i32, sum: i32) -> i32 {
    if sum == amount {
        return 0;
    }
    if sum > amount {
        return std::i32::MAX;
    }

    let mut ans = std::i32::MAX;
    for i in coins {
        let mut result = dfs(coins, amount, sum + i);
        if result == std::i32::MAX {
            continue;
        }
        ans = std::cmp::min(result + 1, std::i32::MAX);
    }

    ans
}

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let result = dfs(&coins, amount, 0);
    if result == std::i32::MAX {
        return -1;
    }
    result
}
