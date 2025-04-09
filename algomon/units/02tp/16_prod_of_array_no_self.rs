fn product_of_array_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prod = 1;
    let mut left_prod = vec![];

    for i in &nums {
        left_prod.push(prod);
        prod *= i;
    }

    let mut ans = vec![];

    prod = 1;
    let mut i = nums.len() - 1;
    while i >= 0 {
        ans.insert(0, prod * left_prod[i]);
        prod *= nums[i];

        if i > 0 {
            i -= 1;
        } else {
            break;
        }
    }

    ans
}
