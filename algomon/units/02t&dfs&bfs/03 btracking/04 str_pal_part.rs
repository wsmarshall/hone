fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn dfs(
    n: usize,
    start_index: usize,
    chars: &Vec<&str>,
    path: &mut Vec<String>,
    res: &mut &Vec<Vec<String>>,
) {
    if start_index >= n {
        res.push(path.clone());
        return;
    }

    for i in start_index..n {
        if !is_palindrome(path) {
            return;
        } else {
            path.push(chars[i]);
            dfs(n, start_index + 1, chars, path, res);
            path.pop();
        }
    }
}

fn partition(s: String) -> Vec<Vec<String>> {
    let mut path = Vec::<String>::new();

    let mut chars: Vec<&str> = s.split("").collect();
    chars.remove(0);
    chars.pop();

    let n = chars.len();

    let mut res = Vec::<Vec<String>>::new();

    dfs(n, 0, &chars, &mut path, &mut res);

    res
}

impl Solution {
    fn partition(s: String) -> Vec<Vec<String>> {
        // backtracking
        // 1. define `ans` to hold `candidates`
        let mut ans = Vec::new();
        // 2. define `candidates` to hold all potential candidates (palindromic substring)
        let mut candidates = Vec::new();
        // 3. start backtrack from index 0
        Solution::backtrack(&s, &mut ans, &mut candidates, 0);
        // 4. return ans
        ans
    }

    fn backtrack(
        s: &String,
        ans: &mut Vec<Vec<String>>,
        candidates: &mut Vec<String>,
        start: usize,
    ) {
        // 1. check if the goal is fulfilled,
        // i.e. reaching the end of string in this problem
        if start == s.len() {
            // if so, we push `candidates` to ans since we've processed all characters
            ans.push(candidates.clone());
            return;
        }
        // 2. find all potential candidates
        for i in start..s.len() {
            // we want to test all substrings, each substring is a potential candidate
            // e.g. "aab" -> "a", "a", "b", "ab", "aa", "b", "aab"
            let candidate = &s[start..i + 1];
            // 3. check if the current candidate is palindrome or not
            // if not, then we cannot push to `candidates`
            if !Solution::is_palindrome(candidate) {
                continue;
            }
            // 4. if so, push it to candidates
            candidates.push(candidate.to_string());
            // 5. backtrack with index + 1
            Solution::backtrack(s, ans, candidates, i + 1);
            // 6. remove the current substring from `candidates`
            candidates.pop();
        }
    }

    fn is_palindrome(s: &str) -> bool {
        s.chars().eq(s.chars().rev())
    }
}
