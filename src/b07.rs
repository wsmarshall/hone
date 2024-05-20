/**
 * A mountain array is defined as an array that

    has at least 3 elements
    has an element with the largest value called "peak", with index k. The array elements strictly increase from the first element to A[k], and then strictly decrease from A[k + 1] to the last element of the array. Thus creating a "mountain" of numbers.

That is, given A[0]<...<A[k-1]<A[k]>A[k+1]>...>A[n-1], we need to find the index k. Note that the peak element is neither the first nor the lastIndex of the array.

Find the index of the peak element. Assume there is only one peak element.
*/
use std::cmp::Ordering::*;

pub fn find_peak(list: &[usize]) -> usize {
    let length = list.len();

    let mut left = 0;
    let mut right = length - 1; //guaranteed non-empty array
    let mut mid;

    let last = list[length - 1];
    let mut boundary = length - 1;

    while left <= right {
        println!("just inside while");
        mid = left + ((right - left) / 2); //to avoid overflow
        let num = list[mid];
        match num.cmp(&last) {
            Less => {
                println!("just inside less case");
                boundary = mid;
                if mid < 1 {
                    println!("just inside less if case");
                    //avoids underflow from usize indexing
                    //particularly when right = 0
                    break;
                } else {
                    println!("just inside less else case");
                    println!("left, right, mid = {}, {}, {}", left, right, mid);
                    right = mid - 1
                }
            }
            Greater | Equal => {
                println!("just inside greater case");
                left = mid + 1;
            }
        }
    }

    boundary
}
