use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let amt: usize = <i32 as TryInto<usize>>::try_into(amount).unwrap();
    let mut memo: Vec<i32> = vec![i32::MAX - 10; amt + 1];
    memo[0] = 0;
    for i in 1..=memo.len() {
        for j in 0..coins.len() {
            if coins[j] <= i.try_into().unwrap() {
                memo[i] = cmp::min(
                    memo[i],
                    memo[i - <i32 as TryInto<usize>>::try_into(coins[j]).unwrap()] + 1,
                );
            }
        }
    }
    memo[amt]
}
