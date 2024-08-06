impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        /* we will iterate through the array two times:
        once in the forward direction, giving the multiplication of all the "pre i" entries
        and similarly in the backward, for all the "post i" entries

        then multiply both those numbers in a carefully placed output array
        and return
        */
        let mut ans = Vec::new();
        let mut acc = 1;
        nums.iter().for_each(|x| {
            ans.push(acc);
            acc *= x;
        });
        acc = 1;
        //has to be iter -> enumerate -> rev
        //because enumerate goes left to right natively
        nums.iter().enumerate().rev().for_each(|(i, x)| {
            ans[i] *= acc;
            acc *= x;
        });

        ans
    }
}
