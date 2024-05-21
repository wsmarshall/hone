/**
 * finds the first true (if present) in a sorted boolean array
 * (all falses, then all trues from left to right)
 */

pub fn find_boundary(list: &[bool]) -> Option<usize> {
    //returns option Some with wrapped index of target boundary
    //or option None if not found
    let mut left: usize = 0;

    if list.is_empty() {
        //accounting for the empty array input
        return None;
    }
    let mut right: usize = list.len() - 1;

    let mut mid: usize;

    let mut boundary_index_initialized: bool = false;
    //initialized for compiler only
    let mut boundary_index: usize = 0;

    while left <= right {
        mid = left + ((right - left) / 2); //to avoid overflow
        match list[mid] {
            true => {
                if !boundary_index_initialized {
                    //initialize if false
                    boundary_index_initialized = true;
                }
                boundary_index = mid;
                if mid < 1 {
                    //avoids underflow from usize indexing
                    //particularly when right = 0
                    break;
                } else {
                    // println!("left: {}, mid: {} right: {}", left, mid, right);
                    right = mid - 1;
                }
            }
            false => left = mid + 1,
        }
    }

    if !boundary_index_initialized {
        //value not present in input array
        return None;
    } else {
        return Some(boundary_index);
    }
}
