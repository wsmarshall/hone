use std::collections::HashSet;
use std::error;
use std::io;

fn longest_substring_without_repeating_characters(s: String) -> i32 {
    let mut collection = HashSet::new();
    let mut max_len = 0;

    //"base case" for the empty string
    if s.len() == 0 {
        return max_len;
    }

    let mut left = 0;
    let mut right = 1;
}
