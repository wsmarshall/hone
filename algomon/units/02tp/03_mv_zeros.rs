impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;
        let mut right = 1;

        while right < nums.len() {
            if nums[left] == 0 {
                let temp = nums[left];
                nums[left] = nums[right];
            }
        }
    }
}
