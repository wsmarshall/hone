fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn dfs(
    s: &String,
    answers: &mut Vec<Vec<String>>,
    candidates: &mut Vec<String>,
    start_index: usize,
) {
    if start_index == s.len() {
        answers.push(candidates.clone());
        return;
    }

    for i in start_index..s.len() {
        let candidate = &s[start_index..i + 1];

        if !is_palindrome(candidate) {
            continue;
        }

        candidates.push(candidate.to_string());
        dfs(s, answers, candidates, i + 1);
        candidates.pop();
    }
}

fn partition(s: String) -> Vec<Vec<String>> {
    let mut candidates = Vec::<String>::new();

    let mut answers = Vec::<Vec<String>>::new();

    dfs(&s, &mut answers, &mut candidates, 0);

    answers
}
