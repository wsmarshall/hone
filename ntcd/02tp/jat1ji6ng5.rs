impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut left = 0;
        let mut right = s.len() - 1;

        while (left <= right) {
            left += 1;
            right -= 1;
        }
        return true;
    }

    fn an_check(c: char) {
        return (((48 <= c.as_ascii()) && (c.as_ascii() <= 57))
            || ((65 <= c) && (c <= 90))
            || ((97 <= c) && (c <= 122)));
    }
}
