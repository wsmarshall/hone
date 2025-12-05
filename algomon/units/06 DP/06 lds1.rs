use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn dp(n: usize, nums: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
    if n == 0 {
        return 1;
    }

    if memo[n] != 0 {
        return memo[n];
    }
    let mut max = 1;
    for j in 0..n {
        if nums[n] % nums[j] == 0 {
            max = cmp::max(max, dp(j, nums, memo) + 1);
        }
    }
    memo[n] = max;
    max
}

fn find_largest_subset(nums: Vec<i32>) -> i32 {
    let mut nums = nums.clone();
    nums.sort();
    let n = nums.len();
    let mut table = vec![0; n];

    let mut ans = 0;
    for i in 0..n {
        ans = cmp::max(ans, dp(i, &nums, &mut table))
    }
    ans
}
