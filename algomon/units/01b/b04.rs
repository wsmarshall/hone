/**
 * first_occurrence: given an array of integers sorted in increasing order and a target,
 * find the index of the first element in the array that is equal to the target
*/

pub fn find_first_occurrence(arr: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len();

    let mut index: i32 = -1;
    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            right = mid;
            index = mid as i32;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            //means arr[mid] > target
            right = mid;
        }
    }
    index
}
