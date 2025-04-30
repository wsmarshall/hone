fn dfs(chars: &Vec<char>, ans: &mut Vec<String>, path: &mut String, used: &mut Vec<bool>) {
    if path.len() == chars.len() {
        ans.push(path.clone());
        return;
    }

    for i in 0..chars.len() {
        if used[i] {
            continue;
        }
        path.push(chars[i]);
        used[i] = true;
        dfs(chars, ans, path, used);
        path.pop();
        used[i] = false;
    }
}

fn permutations(letters: String) -> Vec<String> {
    let mut ans: Vec<String> = vec![];
    let mut path = String::new();
    let chars: Vec<char> = letters.chars().collect();
    let mut used = vec![false; letters.len()];

    dfs(&chars, &mut ans, &mut path, &mut used);

    ans
}
