use std::error;
use std::io;
use std::str::FromStr;

fn can_partition(nums: Vec<i32>) -> bool {
    let total: i32 = nums.iter().sum();
    if total % 2 == 1 {
        return false;
    } else {
        let target = total / 2;
        let n: usize = nums.len();
        let mut dp =
            vec![vec![false; <i32 as TryInto<usize>>::try_into(target).unwrap() + 1]; n + 1];
        //return target_exists(&nums, target.try_into().unwrap(), 0, n, &mut dp);
        dp[0][0] = true;

        for i in 0..=n {
            dp[i][0] = true;
        }

        for i in 1..=n {
            for j in 1..=target as usize {
                if (j as i32) < nums[i - 1] {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j]
                        || dp[i - 1][j - <i32 as TryInto<usize>>::try_into(nums[i - 1]).unwrap()];
                }
            }
        }
        return dp[n][target as usize];
    }
}
