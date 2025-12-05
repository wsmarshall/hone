impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;

        for right in 0..nums.len() {
            if nums[right] != 0 {
                nums.swap(left, right);
                left += 1;
            }
        }
    }
}
