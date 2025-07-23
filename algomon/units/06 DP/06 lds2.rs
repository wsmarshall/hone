use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn find_largest_subset(nums: Vec<i32>) -> i32 {
    let mut nums: Vec<i32> = nums.clone();
    nums.sort();
    let n = nums.len();
    let mut dp = vec![0; n + 1];
    let mut max = 1;

    for i in 1..=n {
        let current = nums[i - 1];
        dp[i] = 1; //sets minimum case; subset of 1
        for j in 1..i {
            if current % nums[j - 1] == 0 {
                dp[i] = cmp::max(dp[i], dp[j] + 1);
            }
        }
        max = cmp::max(max, dp[i]);
    }
    max
}
