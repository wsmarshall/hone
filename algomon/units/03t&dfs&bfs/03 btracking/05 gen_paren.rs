fn dfs(n: i32, answers: &mut Vec<String>, path: &mut String, open_count: i32, closed_count: i32) {
    if path.len() == (2 * n).try_into().unwrap() {
        answers.push(path.clone());
        return;
    }

    if open_count < n {
        path.push('(');
        dfs(n, answers, path, open_count + 1, closed_count);
        path.pop();
    }

    if closed_count < open_count {
        path.push(')');
        dfs(n, answers, path, open_count, closed_count + 1);
        path.pop();
    }
}

fn generate_parentheses(n: i32) -> Vec<String> {
    let mut answers: Vec<String> = vec![];
    let mut path = String::new();

    dfs(n, &mut answers, &mut path, 0, 0);

    answers
}
