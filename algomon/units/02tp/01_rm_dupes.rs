use std::error;
use std::io;
use std::str::FromStr;

fn remove_duplicates(arr: &mut Vec<i32>) -> usize {
    let mut left: usize = 0;
    let mut right: usize = 0;

    let mut prev: i32 = arr[right];

    let len = arr.len();

    while right < len {
        if arr[left] == arr[right] {
            right += 1;
        }
    }

    left
}
