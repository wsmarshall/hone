use std::collections::HashSet;
use std::error;
use std::io;

fn longest_substring_without_repeating_characters(s: String) -> i32 {
    let mut max_len: i32 = 0;

    let length = s.len();

    //"base case" for the empty string
    if length == 0 {
        return max_len;
    }

    let mut collection = HashSet::new();
    let chars: Vec<char> = s.chars().collect();
    let mut left = 0;

    for right in 0..length {
        if !collection.contains(&chars[right]) {
            collection.insert(chars[right]);
            max_len = std::cmp::max(max_len, collection.len() as i32);
        } else {
            collection.remove(&chars[left]);
            left += 1;
        }
    }
    max_len
}
