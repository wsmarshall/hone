use std::error;
use std::io;
use std::str::FromStr;

fn container_with_most_water(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut max = 0;
    while left < right {
        let min = std::cmp::min(arr[left], arr[right]);
        let curr = min * min;
        if curr > max {
            max = curr;
        }
        if left < right {
            left += 1;
        }
        if right <= left {
            right -= 1;
        }
    }
    max
}
