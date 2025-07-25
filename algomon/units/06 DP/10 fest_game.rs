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

    for i in 1..=r {
        let left_interval: i64 = f(l, i - 1, dp, target);
        let right_interval: i64 = f(i + 1, r, dp, target);

        let left_mult: i64 = match l {
            0 => 1,
            _ => target[l - 1].into(),
        };

        let rightmost = target.len() - 1;
        let right_mult: i64 = match r {
            rightmost => 1,
            _ => target[r + 1].into(),
        };

        let val: i64 = left_mult * i64::from(target[i]) * right_mult;

        dp[l][r] = cmp::max(dp[l][r], left_interval + right_interval + val);
    }

    dp[l][r]
}
