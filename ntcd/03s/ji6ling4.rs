impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new();

        for c in s.chars() {
            if (c == '(' || c == '{' || c == '[') {
                stack.push(c);
                continue;
            }

            if c == ')' {
                if !(stack.pop().unwrap_or('0') == '(') {
                    return false;
                }
                continue;
            }
            if c == '}' {
                if !(stack.pop().unwrap_or('0') == '{') {
                    return false;
                }
                continue;
            }
            if c == ']' {
                if !(stack.pop().unwrap_or('0') == '[') {
                    return false;
                }
                continue;
            }
        }
        if stack.len() > 0 {
            return false;
        }
        true
    }
}
