impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        //base case of empty string
        if (s == " " || s == "") {
            return true;
        }
        let input: String = s.chars().filter(|a| a.is_alphabetic()).collect();
        let check: String = input.to_lowercase();

        let mut start = 0;
        let mut end = check.len() - 1;

        while (start <= end) {
            if (check.chars().nth(start) != check.chars().nth(end)) {
                return false;
            }
            start = start + 1;
            end = end - 1;
        }
        return true;
    }
}
