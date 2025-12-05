use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn f(l: i32, r: i32, dp: &mut Vec<Vec<i64>>, target: &Vec<i32>) -> i64 {
    if l > r {
        return 0;
    }

    if dp[l as usize][r as usize] != 0 {
        return dp[l as usize][r as usize];
    }

    for i in l as usize..=r as usize {
        let left_interval: i64 = f(l, (i as i32) - 1, dp, target);
        let right_interval: i64 = f((i + 1).try_into().unwrap(), r, dp, target);

        let left_mult: i64 = match l {
            0 => 1,
            _ => target[(l - 1) as usize].into(),
        };

        let right_mult: i64;
        if r as usize == target.len() - 1 {
            right_mult = 1;
        } else {
            right_mult = target[(r + 1) as usize].into()
        };

        let val: i64 = left_mult * i64::from(target[i]) * right_mult;

        dp[l as usize][r as usize] = cmp::max(
            dp[l as usize][r as usize],
            left_interval + right_interval + val,
        );
    }

    dp[l as usize][r as usize]
}

fn festival_game(target: Vec<i32>) -> i64 {
    let n: usize = target.len();
    let mut dp: Vec<Vec<i64>> = vec![vec![0; n]; n];

    f(0, (n - 1).try_into().unwrap(), &mut dp, &target)
}
