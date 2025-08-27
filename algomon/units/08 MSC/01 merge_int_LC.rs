use std::cmp;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by_key(|interval| interval[0]);

    let mut answers = Vec::with_capacity(intervals.len() + 1);
    let mut current = vec![intervals[0][0], intervals[0][1]];

    for i in intervals.into_iter().skip(1) {
        if current[1] >= i[0] {
            current[1] = cmp::max(current[1], i[1]);
        } else {
            answers.push(current);
            current = i;
        }
    }

    answers.push(current);
    answers
}
