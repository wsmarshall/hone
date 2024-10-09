/**
 * first_not_smaller: given an array of integers
 * sorted in increasing order and a target,
 * find the index of the first element in the array
 * that is greater than or equal to the target
 * assume it is guaranteed to find such
*/

pub fn first_not_smaller(arr: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len();

    let mut index: i32 = -1;

    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] >= target {
            right = mid;
            index = mid as i32;
        } else {
            left = mid + 1;
        }
    }
    index
}
