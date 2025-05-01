fn dfs(s: &str, words: &Vec<String>, start_index: usize) -> bool {
    if start_index == s.len() {
        return true;
    }

    let mut ans = false;
    for i in words {
        if s[start_index..].starts_with(i) {
            ans = ans || dfs(s, words, start_index + i.len());
        }
    }

    ans
}

fn word_break(s: String, words: Vec<String>) -> bool {
    dfs(&s, &words, 0)
}
