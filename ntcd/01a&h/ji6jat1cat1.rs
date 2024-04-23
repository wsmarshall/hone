use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hm = HashMap::new();

        for element in nums.iter() {
            let result = hm.insert(element, 1);
            if (!result.is_none()) {
                return true;
            }
        }
        return false;
    }
}
