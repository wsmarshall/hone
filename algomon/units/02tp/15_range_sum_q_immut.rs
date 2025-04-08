fn range_sum_query_immutable(nums: Vec<i32>, left: i32, right: i32) -> i32 {
    let mut pre_sums = Vec::new();
    pre_sums.push(0);
    let mut sum = 0;
    for i in nums {
        sum += i;
        pre_sums.push(sum);
    }

    pre_sums[(right + 1) as usize] - pre_sums[left as usize]
}
