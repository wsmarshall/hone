use std::error;
use std::io;
use std::str::FromStr;

fn target_exists(
    nums: &Vec<i32>,
    target: usize,
    current: usize,
    n: usize,
    dp: &mut Vec<Vec<i32>>,
) -> bool {
    if current == target {
        return true;
    }

    if n == 0 || current > target {
        return false;
    }

    if dp[n][current] != -1 {
        if dp[n][current] == 1 {
            return true;
        } else {
            return false;
        }
    }

    let exists: bool = target_exists(
        nums,
        target,
        current + <i32 as TryInto<usize>>::try_into(nums[n - 1]).unwrap(),
        n - 1,
        dp,
    ) || target_exists(nums, target, current, n - 1, dp);

    if exists {
        dp[n][current] = 1;
    } else {
        dp[n][current] = 0;
    }

    exists
}

fn can_partition(nums: Vec<i32>) -> bool {
    let total: i32 = nums.iter().sum();
    if total % 2 == 1 {
        return false;
    } else {
        let target = total / 2;
        let n: usize = nums.len();
        let mut dp = vec![vec![-1; <i32 as TryInto<usize>>::try_into(total).unwrap() + 1]; n + 1]; //-1 for not computed, 0 for false, 1 for true
        return target_exists(&nums, target.try_into().unwrap(), 0, n, &mut dp);
    }
}
