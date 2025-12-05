fn is_pal(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn dfs(s: &str, answers: &mut Vec<Vec<String>>, path: &mut Vec<String>, start_index: usize) {
    if start_index == s.len() {
        answers.push(path.clone());
        return;
    }

    for i in start_index..s.len() {
        if is_pal(&s[start_index..i + 1]) {
            path.push(s[start_index..i + 1].to_string());
            dfs(s, answers, path, i + 1);
            path.pop();
        }
    }
}

fn partition(s: String) -> Vec<Vec<String>> {
    let mut answers: Vec<Vec<String>> = vec![];
    let mut path: Vec<String> = vec![];

    dfs(&s, &mut answers, &mut path, 0);

    answers
}
