use std::error;
use std::fmt::Display;
use std::io;

const SIZE: usize = 26;

fn find_all_anagrams(original: String, check: String) -> Vec<i32> {
    let mut indices = vec![];

    let idx = |c: u8| (c - 'a' as u8) as usize;

    let mut anagram = [0u8; SIZE];
    for c in check.chars() {
        anagram[idx(c as u8)] += 1;
    }

    let mut window = [0u8; SIZE];
    let mut left = 0;

    let og: &[u8] = original.as_bytes();
    for right in 0..og.len() {
        window[idx(og[right])] += 1;
        if right >= check.len() - 1 {
            if anagram == window {
                indices.push(left as i32);
            }
            window[idx(og[left])] -= 1;
            left += 1;
        }
    }

    indices
}
