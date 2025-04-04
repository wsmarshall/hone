use std::error;
use std::io;
use std::str::FromStr;

fn subarray_sum_shortest(nums: Vec<i32>, target: i32) -> i32 {
    let length = nums.len();

    //the base case of the empty array
    if length == 0 {
        return 0;
    }

    let mut left = 0;
    let mut right = 0;
    let mut min_len: i32 = length as i32 + 1;
    let mut window_sum: i32 = 0;

    for right in 0..length {
        window_sum += nums[right];
        while window_sum >= target && right >= left {
            min_len = std::cmp::min(min_len, (right - left + 1) as i32);
            window_sum -= nums[left];
            left += 1;
        }
    }

    min_len
}
