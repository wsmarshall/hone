use std::collections::VecDeque;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = [-1; nums.len()];
    let mut mstack: VecDeque<(usize, i32)> = VecDeque::new();

    for (i, el) in nums.iter().enumerate() {
        while !mstack.is_empty() && el > &mstack.back().unwrap().1 {
            let current = mstack.pop_back().unwrap();
            results[current.0] = el;
        }
        mstack.push_back((i, *el));
    }

    results
}
