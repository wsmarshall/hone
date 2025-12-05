fn subarray_sum_fixed(nums: Vec<i32>, k: i32) -> i32 {
    let len: usize = nums.len();
    let mut left: usize = 0;
    let mut right: usize = (k - 1).try_into().unwrap();
    let mut max_sum: i32 = 0;
    for i in 0..k as usize {
        max_sum += nums[i];
    }
    while right < len - 1 {
        let mut sum = max_sum;
        sum -= nums[left];
        left += 1;
        right += 1;
        sum += nums[right];

        max_sum = std::cmp::max(sum, max_sum);
    }

    max_sum
}
