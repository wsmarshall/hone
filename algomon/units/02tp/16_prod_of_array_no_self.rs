fn product_of_array_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prod = 1;
    let mut left_prod = vec![];

    for i in &nums {
        left_prod.push(prod);
        prod *= i;
    }

    let mut ans = vec![];

    prod = 1;
    for (i, el) in nums.iter().rev().enumerate() {
        ans.push(prod * left_prod[i]);
        prod *= el;
    }

    ans
}
