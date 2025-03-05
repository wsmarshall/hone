fn dfs(
    n: i32,
    path: &mut String,
    results: &mut Vec<String>,
    start_index: i32,
    open_count: i32,
    close_count: i32,
) {
    if start_index == 2 * n {
        results.push(path.clone());
    }

    if open_count < n {
        path.push('(');
        dfs(
            n,
            path,
            results,
            start_index + 1,
            open_count + 1,
            close_count,
        );
        path.pop();
    }

    if close_count < open_count {
        path.push(')');
        dfs(
            n,
            path,
            results,
            start_index + 1,
            open_count,
            close_count + 1,
        );
        path.pop();
    }
}

fn generate_parentheses(n: i32) -> Vec<String> {
    let mut results = Vec::<String>::new();
    let mut path = String::new();

    dfs(n, &mut path, &mut results, 0, 0, 0);

    results
}
