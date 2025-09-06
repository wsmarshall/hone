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
        while !track.is_empty() && track.back().unwrap().1 <= *el {
            track.pop_back();
        }
        track.push_back((i, *el));
        if i as i32 - k >= 0 {
            if track.front().unwrap().0 == i - k as usize {
                track.pop_front();
            }
        }

        if i >= (k - 1).try_into().unwrap() {
            maxes.push(track.front().unwrap().1);
        }
    }

    maxes
}
