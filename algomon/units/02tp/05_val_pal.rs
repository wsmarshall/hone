fn is_palindrome(s: String) -> bool {
    let test: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = test.len() - 1;
    while left < right {
        if !test[left].is_alphabetic() {
            left += 1;
            continue;
        }
        if !test[right].is_alphabetic() {
            right -= 1;
            continue;
        }
        if test[left].to_lowercase().to_string() != test[right].to_lowercase().to_string() {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
