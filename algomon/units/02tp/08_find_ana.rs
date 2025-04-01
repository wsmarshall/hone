use std::collections::HashSet;
use std::error;
use std::fmt::Display;
use std::io;

fn find_all_anagrams(original: String, check: String) -> Vec<i32> {
    const SIZE: usize = 26;

    fn find_all_anagrams(original: String, check: String) -> Vec<i32> {
        let mut indices = vec![];

        let idx = |c: u8| (c - 'a' as u8) as usize;

        let mut anagram = [0u8; SIZE];
        for c in check.chars() {
            anagram[idx(c as u8)] += 1;
        }

        indices
    }
}
