use std::error;
use std::io;
use std::str::FromStr;

fn target_exists(nums: &Vec<i32>, target: i32, current: i32, n: usize) -> bool {
    if current == target {
        return true;
    }

    if n == 0 || current > target {
        return false;
    }

    let exists: bool = target_exists(nums, target, current + nums[n - 1], n - 1)
        || target_exists(nums, target, current, n - 1);

    exists
}

fn can_partition(nums: Vec<i32>) -> bool {
    let total: i32 = nums.iter().sum();
    if total % 2 == 1 {
        return false;
    } else {
        let target = total / 2;
        return target_exists(&nums, target, 0, nums.len());
    }
}
