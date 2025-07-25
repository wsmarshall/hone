use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn f(l: usize, r: usize, dp: &mut Vec<Vec<i64>>, target: &Vec<i32>) -> i64 {
    if l > r {
        return 0;
    }

    if dp[l][r] != 0 {
        return dp[l][r];
    }

    for i in l..=r {
        let leftmost = match i {
            0 => 0,
            _ => i - 1,
        };
        let left_interval: i64 = f(l, leftmost, dp, target);
        let right_interval: i64 = f(i + 1, r, dp, target);

        let left_mult: i64 = match l {
            0 => 1,
            _ => target[l - 1].into(),
        };

        let right_mult;
        if r == target.len() - 1 {
            right_mult = 1;
        } else {
            right_mult = target[r + 1].into()
        };

        let val: i64 = left_mult * i64::from(target[i]) * right_mult;

        dp[l][r] = cmp::max(dp[l][r], left_interval + right_interval + val);
    }

    dp[l][r]
}

fn festival_game(target: Vec<i32>) -> i64 {
    let n: usize = target.len();
    let mut dp: Vec<Vec<i64>> = vec![vec![0; n]; n];

    f(0, n - 1, &mut dp, &target)
}
