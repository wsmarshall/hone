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

    let mut collection = HashMap::new();
    let chars: Vec<char> = s.chars().collect();
    let mut left = 0;

    for right in 0..length {
        *collection.entry(chars[right]).or_insert(0) += 1;
        while collection.get(&chars[right]) > Some(&1) {
            collection.entry(chars[left]).and_modify(|c| *c -= 1);
            left += 1;
        }
        max_len = std::cmp::max(max_len, (right - left + 1) as i32);
    }

    max_len
}

//alternate solution with HashSet approach:
/**
 * fn longest_substring_without_repeating_characters(s: String) -> i32 {
    let length = s.len();
    if length == 0 {
        return 0;
    }
    
    let string: Vec<char> = s.chars().collect();
    let mut acc = HashSet::new();
    let mut max_len: i32 = 0;
    let mut left = 0;
    for right in 0..length {
        while acc.contains(&string[right]) {
            acc.remove(&string[left]);
            left += 1;
        }
        
        acc.insert(&string[right]);
        max_len = std::cmp::max(max_len, (right - left + 1) as i32);   
    }
    
    max_len
    
}
 */
