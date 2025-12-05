fn two_sum_sorted(arr: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut sum = arr[left] + arr[right];

    while sum != target {
        if sum < target {
            left += 1;
        }
        if sum > target {
            right -= 1;
        }
        sum = arr[left] + arr[right];
    }
    vec![left as i32, right as i32]
}
