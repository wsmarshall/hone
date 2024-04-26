/**
 * first_not_smaller: given an array of integers sorted in increasing order and a target,
 * find the index of the first element in the array that is greater than or equal to the target
 * assume it is guaranteed to find such
*/
use std::cmp::Ordering::*;

pub fn first_not_smaller(list: &[usize], target: usize) -> usize {
    let length = list.len();
    //guarantees mean no need to check for empty list

    let mut left = 0;
    let mut right = length - 1; //guaranteed non-empty array
    let mut mid = left + ((right - left) / 2);

    let mut current: usize = 0;

    while left <= right {
        mid = left + ((right - left) / 2); //to avoid overflow
        match list[mid].cmp(&target) {
            Equal => {
                current = mid;
                right = mid - 1;
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

    current
}
