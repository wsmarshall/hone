use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn min_coins(coins: &Vec<i32>, amount: i32, sum: i32) -> i32 {
    if sum == amount {
        return 0;
    }

    if sum > amount {
        return i32::MAX;
    }

    let mut ans = i32::MAX;
    for coin in coins {
        let result = min_coins(coins, amount, sum + coin);
        if result == i32::MAX {
            continue;
        }
        ans = cmp::min(ans, result + 1);
    }

    return ans;
}

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let result = min_coins(&coins, amount, 0);
    if result == i32::MAX {
        return -1;
    } else {
        return result;
    }
}
