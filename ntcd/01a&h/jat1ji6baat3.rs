impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = HashSet::<i32>::from_iter(nums.clone().into_iter());
        let mut ans = 0;

        for i in set.iter() {
            let mut length = 1;
            //possible start to a longest consecutive number sequence
            if !set.contains(&(i - 1)) {
                while set.contains(&(*i + length)) {
                    length += 1;
                }
                if length > ans {
                    ans = length;
                }
            }
        }
        ans
    }
}
