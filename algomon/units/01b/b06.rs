/**
 * A sorted array of unique integers was rotated at an unknown pivot.
 * For example, [10, 20, 30, 40, 50] becomes [30, 40, 50, 10, 20].
 * Find the index of the minimum element in this array.
*/

fn find_min_rotated(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;

    let mut index = arr.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < arr[index] {
            right = mid;
            index = mid;
        } else {
            left = mid + 1;
        }
    }
    index as i32
}
