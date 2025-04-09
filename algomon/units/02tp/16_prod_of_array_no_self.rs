fn product_of_array_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prod = 1;
    for i in &nums {
        if i != &0 {
            prod *= i;
        }
    }
    let mut ans = Vec::<i32>::new();
    for i in nums {
        if i != 0 {
            ans.push(prod / i);
        } else {
            ans.push(prod);
        }
    }

    ans
}
