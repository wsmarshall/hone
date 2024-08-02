use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for (i, n) in nums.iter().enumerate() {
            let j = target - n;
            match map.get(&j) {
                Some(e) => return vec![i as i32, *map.get(&j).unwrap()],
                None => map.insert(n, i as i32),
            };
        }

        //unreachable section of code
        vec![]
    }
}
