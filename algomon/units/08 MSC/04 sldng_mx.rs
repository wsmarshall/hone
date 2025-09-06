use std::collections::VecDeque;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn sliding_window_maximum(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut maxes: Vec<i32> = vec![];

    //monotonically decreasing "stack"
    let mut track: VecDeque<(usize, i32)> = VecDeque::new();

    for (i, el) in nums.iter().enumerate() {
        while !track.is_empty() && track.back() <= el {
            track.pop_back();
        }
        track.push_back((i, el));

        if track.front() == i - k {
            track.pop_front();
        }
    }

    maxes
}
