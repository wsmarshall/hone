fn dfs(coins: &Vec<i32>, amount: i32, sum: i32, memo: &mut Vec<i32>) -> i32 {
    if sum == amount {
        return 0;
    }
    if sum > amount {
        return std::i32::MAX;
    }
    if memo[sum as usize] != -1 {
        return memo[sum as usize];
    }

    let mut ans = std::i32::MAX;
    for i in coins {
        let mut result = dfs(coins, amount, sum + i, memo);
        if result == std::i32::MAX {
            continue;
        }
        ans = std::cmp::min(result + 1, ans);
    }
    memo[sum as usize] = ans;
    ans
}

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut memo: Vec<i32> = vec![-1; (amount + 1).try_into().unwrap()];
    let result = dfs(&coins, amount, 0, &mut memo);
    if result == std::i32::MAX {
        return -1;
    }
    result
}
