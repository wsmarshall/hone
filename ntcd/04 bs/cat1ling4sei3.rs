impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = nums.len() / 2;
        let mut left = 0;
        let mut right = nums.len();

        while (left <= right) {
            if nums[i] == target {
                return i as i32;
            } else if nums[i] < target {
                left = i + 1;
                i = (left + right) / 2;
            }
            //nums[i] > target
            else {
                right = i - 1;
                i = (left + right) / 2;
            }
        }

        -1
    }
}
