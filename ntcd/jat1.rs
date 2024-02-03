use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //stores leftover (target minus index value), index resp.
        let mut map = HashMap::<i32, i32>::new();

        //for the answers
        let mut ans = Vec::<i32>::new();

        for i in 0..nums.len() {
            let current = nums[i];
            if (map.contains_key(&current)) {
                ans.push(*map.get(&current).unwrap_or(&-1));
                ans.push(i as i32);
                return ans;
            }
            let dif = target - current;
            map.insert(dif, i as i32);
        }

        //empty vector if no answer present
        return Vec::<i32>::new();
    }
}
