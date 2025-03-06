fn dfs(chars: &Vec<char>, candidates: &mut String, answers: &mut Vec<String>, start_index: usize) {
    if candidates.len() == chars.len() {
        answers.push(candidates.clone());
        return;
    }

    for c in chars {
        candidates.push(*c);
        dfs(chars, candidates, answers, start_index + 1);
        candidates.pop();
    }
}

fn permutations(letters: String) -> Vec<String> {
    let mut candidates = String::new();
    let mut answers = Vec::<String>::new();

    let chars: Vec<char> = letters.chars().collect();

    dfs(&chars, &mut candidates, &mut answers, 0);

    answers
}
