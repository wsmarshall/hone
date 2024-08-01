use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::<i32>::new();
        for i in nums {
            if set.contains(&i) {
                return true;
            } else {
                set.insert(i);
            }
        }
        false
    }
}
