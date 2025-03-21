use std::collections::HashSet;
use std::error;
use std::fmt::Display;
use std::io;

fn find_all_anagrams(original: String, check: String) -> Vec<i32> {
    let mut indices = vec![];
    let mut check = HashSet::new();
    let chars: Vec<char> = check.chars().collect();

    for i in chars {
        check.insert(i);
    }

    indices
}
