/**
 * square root estimation:Given an integer, find its square root
 *  without using the built-in square root function.
 * Only return the integer part (truncate the decimals).
*/
use std::cmp::Ordering::*;

pub fn square_root(list: &[usize], target: usize) -> Option<usize> {
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
