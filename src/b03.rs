/**
 * first_not_smaller: given an array of integers sorted in increasing order and a target,
 * find the index of the first element in the array that is greater than or equal to the target
 * assume it is guaranteed to find such
*/

pub fn first_not_smaller(list: &[usize], target: usize) -> usize {
    let length = list.len();

    let mut left = 0;
    let mut right = length - 1; //guaranteed non-empty array
    let mut mid = right / 2;

    let mut current: usize = 0;
    while left <= right {
        if list[mid] >= target {
            current = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
        mid = (left + (right - left)) / 2;
    }

    current
}
