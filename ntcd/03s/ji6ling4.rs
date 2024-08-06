impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                ')' | '}' | ']' if Some(c) != stack.pop() => return false,
                _ => (),
            }
        }
        stack.len() == 0
    }
}
