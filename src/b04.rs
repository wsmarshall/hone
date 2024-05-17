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
    let mut mid;

    let mut current: Option<usize> = None;
    let not_found: Option<usize> = None;

    while left <= right {
        println!("just inside while");
        mid = left + ((right - left) / 2); //to avoid overflow
        match list[mid].cmp(&target) {
            Equal => {
                println!("just inside equal case");
                current = Option::Some(mid);
                if mid < 1 {
                    println!("just inside equal if case");
                    //avoids underflow from usize indexing
                    //particularly when right = 0
                    break;
                } else {
                    println!("just inside equal else case");
                    right = mid - 1
                }
            }
            Less => left = mid + 1,
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
