fn dfs(s: &str, words: &Vec<String>, memo: &mut Vec<Option<bool>>, start_index: usize) -> bool {
    if start_index == s.len() {
        return true;
    }

    if let Some(b) = memo[start_index] {
        return b;
    }

    let mut ans = false;
    for i in words {
        if s[start_index..].starts_with(i) {
            ans = ans || dfs(s, words, memo, start_index + i.len());
        }
    }
    memo[start_index] = Some(ans);
    ans
}

fn word_break(s: String, words: Vec<String>) -> bool {
    let mut memo = vec![None; s.len()];
    dfs(&s, &words, &mut memo, 0)
}
