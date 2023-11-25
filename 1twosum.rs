use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut entries = HashMap::new();
        let length = nums.len();
        let result: Vec<i32> = Vec::new();  

        for i in 0..length {
            let complement: i32 = target - (nums.get(i) as i32);
            if entries.contains_key(&complement) {
                result.push(i as i32);
                result.push(entries.get(&complement) as i32);
                break;
            }
            entries.insert(nums.get(i) as i32,i as i32);
        } 

        result
    }
}