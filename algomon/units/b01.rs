use std::cmp::Ordering::*;

pub fn vanilla_binary(list: &[usize], target: usize) -> Option<usize> {
    //returns option Some with wrapped index of target
    //or option None if not found
    let mut left: usize = 0;
    if list.is_empty() {
        //accounting for the empty array input
        return None;
    }
    let mut right: usize = list.len() - 1;

    let mut mid: usize;

    while left <= right {
        mid = left + ((right - left) / 2); //to avoid overflow
        match list[mid].cmp(&target) {
            Equal => return Some(mid),
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
    //value not present in input array
    None
}
