use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn dp(n: usize, nums: &Vec<i32>, table: &mut Vec<i32>) -> i32 {
    if n == 0 {
        return 0;
    }
}

fn find_largest_subset(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut table = vec![0; n + 1];

    dp(n, &nums, &mut table)
}
