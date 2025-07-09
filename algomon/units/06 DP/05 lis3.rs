use std::error;
use std::io;
use std::str::FromStr;

fn bs(nums: &Vec<i32>) -> usize {
    let mut left = 0;
    let mut right = nums.len();

    let mut ans = left;

    while left < right {
        let mid = left + (right - left) / 2;
        if mid > ans {
            ans = mid;
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    ans
}

fn longest_sub_len(nums: Vec<i32>) -> i32 {
    let mut seq: Vec<i32> = vec![];
    for i in nums {
        if Some(&i) > seq.last() {
            seq.push(i);
        } else if Some(&i) == seq.last() {
            continue;
        } else {
            let greater = bs(&seq);
            seq[greater] = i;
        }
    }
    seq.len().try_into().unwrap()
}
