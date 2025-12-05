use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn min_coins(coins: &Vec<i32>, amount: i32, sum: i32, memo: &mut Vec<i32>) -> i32 {
    if sum == amount {
        return 0;
    }

    if sum > amount {
        return i32::MAX;
    }

    if memo[<i32 as TryInto<usize>>::try_into(sum).unwrap()] != -1 {
        return memo[<i32 as TryInto<usize>>::try_into(sum).unwrap()];
    }

    let mut ans = i32::MAX;
    for coin in coins {
        let result = min_coins(coins, amount, sum + coin, memo);
        if result == i32::MAX {
            continue;
        }
        ans = cmp::min(ans, result + 1);
    }
    memo[<i32 as TryInto<usize>>::try_into(sum).unwrap()] = ans;
    return ans;
}

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amt: usize = <i32 as TryInto<usize>>::try_into(amount).unwrap() + 1;
    let mut memo: Vec<i32> = vec![-1; amt];
    let result = min_coins(&coins, amount, 0, &mut memo);
    if result == i32::MAX {
        return -1;
    } else {
        return result;
    }
}
