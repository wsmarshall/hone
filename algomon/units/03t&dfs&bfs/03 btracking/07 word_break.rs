fn dfs(s: &str, words: &Vec<String>, start_index: usize, memo: &mut Vec<Option<bool>>) -> bool {
    if start_index == s.len() {
        return true;
    }

    if let Some(b) = memo[start_index] {
        return b;
    }

    let mut ans = false;
    for i in words {
        if s[start_index..].starts_with(i) {
            ans = ans || dfs(s, words, start_index + i.len(), memo);
        }
    }
    memo[start_index] = Some(ans);
    ans
}

fn word_break(s: String, words: Vec<String>) -> bool {
    let mut memo = vec![None; s.len()];
    dfs(&s, &words, 0, &mut memo)
}
