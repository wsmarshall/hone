use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn longest_sub_len(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut bu: Vec<i32> = vec![0; n + 1];

    let mut ans: i32 = 0;
    for i in 1..n + 1 {
        let n_i = nums[i - 1];
        bu[i] = 1;

        for j in 1..i {
            let n_j = nums[j - 1];
            if n_j < n_i {
                bu[i] = cmp::max(bu[i], bu[j] + 1);
            }
        }

        ans = cmp::max(ans, bu[i]);
    }

    ans
}
