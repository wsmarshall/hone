use std::error;
use std::io;
use std::str::FromStr;

fn can_partition(nums: Vec<i32>) -> bool {
    let total: i32 = nums.iter().sum();
    if total % 2 == 1 {
        return false;
    } else {
        let target = total / 2;
        let n: usize = nums.len();
        let mut previous = vec![false; <i32 as TryInto<usize>>::try_into(target).unwrap() + 1];
        previous[0] = true;
        let mut current = previous.clone();

        for i in 1..=n {
            for j in 1..=target as usize {
                if (j as i32) < nums[i - 1] {
                    current[j] = previous[j];
                } else {
                    current[j] = previous[j]
                        || previous[j - <i32 as TryInto<usize>>::try_into(nums[i - 1]).unwrap()];
                }
            }
            // println!("previous: {:?}", previous);
            previous = current.clone();
        }
        return previous[target as usize];
    }
}
