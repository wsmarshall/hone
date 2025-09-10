use std::collections::VecDeque;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = vec![0; t.len()];

    let mut stack: VecDeque<(usize, i32)> = VecDeque::new();

    stack.push_back((0usize, t[0]));

    for (i, el) in t.iter().skip(1).enumerate() {
        while !stack.is_empty() && el > &stack.back().unwrap().1 {
            let current = stack.pop_back().unwrap();
            results[current.0] = ((i - current.0).try_into().unwrap());
        }
        stack.push_back((i, *el));
    }

    results
}
