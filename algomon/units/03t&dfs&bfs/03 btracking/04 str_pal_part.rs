fn is_pal(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn dfs(s: &String, start_index: usize, ans: &mut Vec<Vec<String>>, path: &mut Vec<String>) {
    if start_index == s.len() {
        ans.push(path.clone());
        return;
    }

    for i in start_index..s.len() {
        let candidate = &s[start_index..i + 1];

        if !is_pal(candidate) {
            continue;
        }

        path.push(candidate.to_string());
        dfs(&s, i + 1, ans, path);
        path.pop();
    }
}

fn partition(s: String) -> Vec<Vec<String>> {
    let mut ans: Vec<Vec<String>> = vec![];
    let mut path: Vec<String> = vec![];

    dfs(&s, 0, &mut ans, &mut path);

    ans
}
