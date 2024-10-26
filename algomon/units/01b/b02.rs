/**
 * Finding the First True in a Sorted Boolean Array

An array of boolean values is divided into two sections:
 The left section consists of all false,
 and the right section consists of all true.

 Find the First True in a Sorted Boolean Array of the right section,
  i.e., the index of the first true element.

  If there is no true element, return -1.

EXAMPLE:
Input: arr = [false, false, true, true, true]

Output: 2

Explanation: The first true's index is 2.
 */

fn find_boundary(arr: Vec<bool>) -> i32 {
    let mut left = 0;
    let mut right = arr.len();

    let mut index: i32 = -1;

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] {
            index = mid as i32;
            right = mid;
        } else if !arr[mid] {
            left = mid + 1;
        }
    }

    index
}
