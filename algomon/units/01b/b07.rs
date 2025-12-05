/**
 * A mountain array is defined as an array that

    has at least 3 elements
    has an element with the largest value called "peak", with index k. The array elements strictly increase from the first element to A[k], and then strictly decrease from A[k + 1] to the last element of the array. Thus creating a "mountain" of numbers.

That is, given A[0]<...<A[k-1]<A[k]>A[k+1]>...>A[n-1], we need to find the index k. Note that the peak element is neither the first nor the lastIndex of the array.

Find the index of the peak element. Assume there is only one peak element.
*/
fn peak_of_mountain_array(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;

    let mut index = 0;

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] > arr[mid + 1] {
            index = mid;
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    index as i32
}
