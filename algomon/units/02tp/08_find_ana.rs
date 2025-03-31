use std::collections::HashSet;
use std::error;
use std::fmt::Display;
use std::io;

fn find_all_anagrams(original: String, check: String) -> Vec<i32> {
    let mut indices = vec![];
    let mut reference = HashMap::new();

    let og_chars: Vec<char> = original.chars().collect();
    let check_chars: Vec<char> = check.chars().collect();

    for i in og_chars {
        *reference.entry(i).or_insert(0) += 1;
    }

    let mut left = 0;
    let mut right = check.len() - 1;
    let mut current = HashMap::new();

    for i in left..check.len() {
        *current.entry(check_chars[i]).or_insert(0) += 1;
    }

    while right < check.len() - 1 {
        if current.eq(&reference) {
            indices.push(left.try_into().unwrap());
        }

        if let Some(v) = current.get(&check_chars[left]) {
            if *v == 1 {
                current.remove(&check_chars[left]);
            } else {
                *current.entry(check_chars[left]).or_insert(1) -= 1;
            }
        }
        left += 1;
        right += 1;
        *current.entry(check_chars[right]).or_insert(0) += 1;
    }

    indices
}
