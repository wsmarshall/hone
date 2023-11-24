use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut entries = HashMap::new();
        let length = nums.len() as i32;

        for i in 0..length {
            let complement: i32 = target - *nums.get(i);
            if entries.contains_key(&complement) {
                return vec![i.try_into().unwrap(), entries.get(&complement)];
            }
            entries.insert(nums.get(i),i);
        } 
    }
}