fn dfs(nums: &Vec<i32>, answers: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, start_index: usize) {
    answers.push(current.clone());
    for i in start_index..nums.len() {
        current.push(nums[i]);
        dfs(nums, answers, current, i + 1);
        current.pop();
    }
}

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut answers: Vec<Vec<i32>> = vec![];
    let mut current: Vec<i32> = vec![];

    dfs(&nums, &mut answers, &mut current, 0);

    answers
}
