impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len();

        while (left < right) {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] < target {
                left = mid + 1;
            }
            //nums[i] > target
            else {
                right = mid;
            }
        }

        -1
    }
}
