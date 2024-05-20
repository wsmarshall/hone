/**
 * square root estimation:Given an integer, find its square root
 *  without using the built-in square root function.
 * Only return the integer part (truncate the decimals).
*/
use std::cmp::Ordering::*;

pub fn square_root(target: usize) -> Option<usize> {
    //creates an array of integers from 0 to n
    let mut list = Vec::new();
    for i in 0..target + 1 {
        list.push(i);
    }
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
