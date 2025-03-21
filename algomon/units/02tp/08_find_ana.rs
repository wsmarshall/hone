use std::collections::HashSet;
use std::error;
use std::fmt::Display;
use std::io;

fn find_all_anagrams(original: String, check: String) -> Vec<i32> {
    let mut indices = vec![];
    let mut reference = HashSet::new();
    let og_chars: Vec<char> = original.chars().collect();
    let check_chars: Vec<char> = check.chars().collect();

    for i in og_chars {
        reference.insert(i);
    }

    let mut left = 0;
    let mut right = original.len();
    let mut current = HashSet::new();

    for i in left..right {
        current.insert(&check_chars[i]);
    }

    while right < check.len() {
        if current.eq(&reference) {
            indices.push(left.try_into().unwrap());
        }
        current.remove(check_chars[left]);
        left += 1;
        current.insert(check_chars[right]);
        right += 1;
    }

    indices
}
