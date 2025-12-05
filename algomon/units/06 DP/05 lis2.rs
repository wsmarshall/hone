use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

//"top-down" approach
fn f(i: usize, nums: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
    if i == 0 {
        return 0;
    }

    if memo[i] != 0 {
        return memo[i];
    }

    let n_i = nums[i - 1];
    let mut ans = 1;
    for j in 1..i {
        let n_j = nums[j - 1];
        let f_of_j = f(j, nums, memo);
        if n_j < n_i {
            ans = cmp::max(ans, f_of_j + 1);
        }
    }

    memo[i] = ans;
    ans
}

fn longest_sub_len(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut memo = vec![0; n + 1];

    f(n, &nums, &mut memo);

    *memo.iter().max().unwrap_or(&0)
}
