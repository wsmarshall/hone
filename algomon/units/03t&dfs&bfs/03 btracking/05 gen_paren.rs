fn dfs(
    n: i32,
    ans: &mut Vec<String>,
    path: &mut String,
    start_index: i32,
    open_count: i32,
    closed_count: i32,
) {
    if start_index == 2 * n {
        ans.push(path.clone());
        return;
    }

    if open_count < n {
        path.push('(');
        dfs(n, ans, path, start_index + 1, open_count + 1, closed_count);
        path.pop();
    }

    if closed_count < open_count {
        path.push(')');
        dfs(n, ans, path, start_index + 1, open_count, closed_count + 1);
        path.pop();
    }
}

fn generate_parentheses(n: i32) -> Vec<String> {
    let mut ans: Vec<String> = vec![];
    let mut path = String::new();

    dfs(n, &mut ans, &mut path, 0, 0, 0);

    ans
}
