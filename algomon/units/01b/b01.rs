pub fn vanilla_binary(nums: Vec<i32>, target: i32) -> i32 {
    //returns index if target found
    //or -1 if not found

    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2; //to avoid overflow type errors
        if nums[mid] == target {
            return mid as i32;
        }
        if nums[mid] < target {
            left = mid + 1;
        }
        //nums[mid] > target
        else {
            right = mid;
        }
    }

    //value not present in input array
    -1
}
