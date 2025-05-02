fn dfs(digits: &str, memo: &mut Vec<i32>, start_index: usize) -> i32 {
    if start_index == digits.len() {
        return 1;
    }

    if memo[start_index] != -1 {
        return memo[start_index];
    }

    let mut ways = 0;

    if digits[start_index..start_index + 1].eq("0") {
        return ways;
    }

    ways += dfs(digits, memo, start_index + 1);

    if start_index + 2 <= digits.len()
        && digits[start_index..start_index + 2].parse::<i32>().unwrap() <= 26
    {
        ways += dfs(digits, memo, start_index + 2);
    }

    memo[start_index] = ways;

    ways
}

fn decode_ways(digits: String) -> i32 {
    let mut memo = vec![-1; digits.len()];
    dfs(&digits, &mut memo, 0)
}
