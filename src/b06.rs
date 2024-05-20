/**
 * A sorted array of unique integers was rotated at an unknown pivot.
 * For example, [10, 20, 30, 40, 50] becomes [30, 40, 50, 10, 20].
 * Find the index of the minimum element in this array.
*/
use std::cmp::Ordering::*;

pub fn find_min_rotated(list: &[usize]) -> usize {
    let mut left = 0;
    let mut right = length - 1; //guaranteed non-empty array
    let mut mid;

    let mut current: Option<usize> = None;
    let not_found: Option<usize> = None;

    while left <= right {
        println!("just inside while");
        mid = left + ((right - left) / 2); //to avoid overflow
        let num = list[mid];
        let square = num * num;
        match square.cmp(&target) {
            //TODO think through this bit
            Equal | Less => {
                println!("just inside equal or less case");
                left = mid + 1;
                current = Option::Some(mid);
            }
            Greater => {
                println!("just inside greater case");
                if right < 1 {
                    println!("just inside greater if case");
                    //avoids underflow from usize indexing
                    //particularly when right = 0
                    break;
                } else {
                    println!("just inside greater else case");
                    right = mid - 1
                }
            }
        }
    }

    match current {
        Some(_) => return current,
        _ => return not_found,
    }
}
