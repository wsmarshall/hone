
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let t: Vec<char> = s
        .chars()
        .filter_map(|c| c.is_ascii_alphanumeric().then(|| c.to_ascii_lowercase()))
        .collect();

    t.iter().eq(t.iter().rev())
    }
}