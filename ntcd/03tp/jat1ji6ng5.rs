extern crate palindrome;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        palindrome::is_palindrome(s)
        let mut left = 0; 
        let length = s.chars().count();
        println!("length: {}", length);
        let mut right = length -1;

        while (left < right && right < length) {
            println!("left, right: {}, {}", left, right);
            while(!Self::an_check(s.as_bytes()[left]) && left < right){
                left += 1;
            }
            while(!Self::an_check(s.as_bytes()[right]) && right < length){
                right -= 1;
            }
            println!("left, right: {}, {}", left, right);
            if( (s.to_lowercase().chars().nth(left)) != 
            (s.to_lowercase().chars().nth(right))){
                return false;
            }
            left += 1;
            right -= 1;
        }
        return true;

    }

    fn an_check(c: u8) -> bool {
        println!("c: {}", c);
        
        (((48 <= c) && (c <= 57))||
        ((65 <= c) && (c <= 90))||
        ((97 <= c) && (c <= 122)))
    }
}