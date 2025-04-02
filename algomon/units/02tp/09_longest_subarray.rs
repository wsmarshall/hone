use std::error;
use std::io;
use std::str::FromStr;

fn subarray_sum_longest(nums: Vec<i32>, target: i32) -> i32 {
    let length = nums.len();
    let mut max = 0;

    if length == 0 {
        return max;
    }

    let mut left = 0;
    let mut right = 1;

    while right < length {
        let window = &nums[left..right];
        let sum: i32 = window.iter().sum();
        if sum <= target {
            max = std::cmp::max(max, (right - left + 1) as i32);
        }
        right += 1;
    }
    max
}
