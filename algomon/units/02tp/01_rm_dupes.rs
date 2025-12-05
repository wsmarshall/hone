use std::error;
use std::io;
use std::str::FromStr;

fn remove_duplicates(arr: &mut Vec<i32>) -> usize {
    let mut left: usize = 0;
    let mut right: usize = 0;

    let len = arr.len();

    while right < len {
        if arr[right] == arr[left] {
            right += 1;
            continue;
        }

        left += 1;
        arr[left] = arr[right];
        right += 1;
    }

    left + 1
}
