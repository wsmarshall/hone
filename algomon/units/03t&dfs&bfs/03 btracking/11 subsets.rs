fn dfs(nums: &Vec<i32>, answers: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, start_index: usize) {
    answers.push(path.clone());
    for i in start_index..nums.len() {
        path.push(nums[i]);
        dfs(nums, answers, path, i + 1);
        path.pop();
    }
}

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut answers: Vec<Vec<i32>> = vec![];
    let mut path: Vec<i32> = vec![];

    dfs(&nums, &mut answers, &mut path, 0);

    answers
}
