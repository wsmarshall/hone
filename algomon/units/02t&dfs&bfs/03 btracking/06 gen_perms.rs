fn dfs(
    chars: &Vec<char>,
    candidates: &mut String,
    answers: &mut Vec<String>,
    start_index: usize,
    used: &mut Vec<bool>,
) {
    if candidates.len() == chars.len() {
        answers.push(candidates.clone());
        return;
    }

    for i in 0..chars.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        candidates.push(chars[i]);
        dfs(chars, candidates, answers, start_index + 1, used);
        used[i] = false;
        candidates.pop();
    }
}

fn permutations(letters: String) -> Vec<String> {
    let mut candidates = String::new();
    let mut answers = Vec::<String>::new();
    let chars: Vec<char> = letters.chars().collect();

    let mut used = Vec::<bool>::new();

    for i in 0..chars.len() {
        used.push(false);
    }
    dfs(&chars, &mut candidates, &mut answers, 0, &mut used);

    answers
}
