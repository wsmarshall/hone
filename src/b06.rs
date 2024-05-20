/**
 * A sorted array of unique integers was rotated at an unknown pivot.
 * For example, [10, 20, 30, 40, 50] becomes [30, 40, 50, 10, 20].
 * Find the index of the minimum element in this array.
*/
use std::cmp::Ordering::*;

pub fn find_min_rotated(list: &[usize]) -> usize {
    let length = list.len();

    let mut left = 0;
    let mut right = length - 1; //guaranteed non-empty array
    let mut mid;

    let last = list[length - 1];
    let mut boundary = length;

    while left <= right {
        println!("just inside while");
        mid = left + ((right - left) / 2); //to avoid overflow
        let num = list[mid];
        match num.cmp(&last) {
            Less => {
                println!("just inside less case");
                boundary = mid;
                if right < 1 {
                    println!("just inside less if case");
                    //avoids underflow from usize indexing
                    //particularly when right = 0
                    break;
                } else {
                    println!("just inside less else case");
                    right = mid - 1
                }
            }
            Greater => {
                println!("just inside greater case");
                left = mid + 1;
            }
            Equal => {
                //this case should not happen by assumption
                continue;
            }
        }
    }

    boundary
}
