/**
 * first_occurrence: given an array of integers sorted in increasing order and a target,
 * find the index of the first element in the array that is equal to the target
*/
use std::cmp::Ordering::*;

pub fn first_occurrence(list: &[usize], target: usize) -> Option<usize> {
    let length = list.len();
    //guarantees mean no need to check for empty list

    let mut left = 0;
    let mut right = length - 1; //guaranteed non-empty array
    let mut mid = left + ((right - left) / 2);

    let mut current: Option<usize> = None;
    let not_found = None;

    while left <= right {
        mid = left + ((right - left) / 2); //to avoid overflow
        match list[mid].cmp(&target) {
            Equal => {
                current = mid;
            }
            Less => left = mid + 1,
            Greater => {
                if right < 1 {
                    //avoids underflow from usize indexing
                    //particularly when right = 0
                    break;
                } else {
                    right = mid - 1
                }
            }
        }
    }

    match current {
        Some => return current,
        _ => return None,
    }
}
