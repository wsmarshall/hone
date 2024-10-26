/**
 * square root estimation:Given an integer, find its square root
 *  without using the built-in square root function.
 * Only return the integer part (truncate the decimals).
*/
use std::cmp::Ordering::*;

pub fn square_root(target: usize) -> Option<usize> {
    let nums: Vec<i32> = (0..n + 1).collect();
    let mut left = 0;
    let mut right = nums.len();

    let mut sqrt: i32 = 0;

    while left < right {
        let mid = left + (right - left) / 2;
        let test = nums[mid] * nums[mid];

        if test == n {
            sqrt = nums[mid];
            return sqrt;
        } else if test < n {
            left = mid + 1;
            sqrt = nums[mid];
        } else {
            right = mid;
        }
    }

    sqrt
}
