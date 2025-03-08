fn dfs(s: &str, words: &Vec<String>, start_index: usize) -> bool {
    
    if start_index == s.len() {
        return true;
    }
    
    for i in words:
        if !i.eq(&s[start_index..(start_index + i.len())]) {
            continue;
        }
        
        return dfs(s, words, start_index + i.len())
    }
}

fn word_break(s: String, words: Vec<String>) -> bool {
    dfs(&s, &words, 0) || false
}