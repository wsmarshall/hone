fn dfs(
    candidates: &Vec<i32>,
    answers: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
    target: i32,
    mut sum: i32,
    start_index: usize,
) {
    if sum == target {
        answers.push(path.clone());
        return;
    }
    //unclear if this is even necessary
    if sum < target {
        for i in start_index..candidates.len() {
            if sum + candidates[i] > target {
                break;
            }
            sum += candidates[i];
            path.push(candidates[i]);
            dfs(candidates, answers, path, target, sum, i);
            path.pop();
            sum -= candidates[i];
        }
    }
}

fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut answers: Vec<Vec<i32>> = vec![];
    let mut path: Vec<i32> = vec![];

    candidates.sort();

    dfs(&candidates, &mut answers, &mut path, target, 0, 0);

    answers
}
