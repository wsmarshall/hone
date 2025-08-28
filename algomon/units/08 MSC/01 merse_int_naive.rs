use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

//doesn't work properly on all tests -- TODO (fix?)

fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort();
    intervals.reverse();

    let mut answers = vec![];
    answers.push(intervals.pop().unwrap());
    while intervals.len() > 0 {
        let next = intervals.pop().unwrap();
        let prev = &answers[answers.len() - 1];
        if next[0] <= prev[1] {
            let new_interval = vec![prev[0], next[1]];
            answers.pop();
            answers.push(new_interval);
        } else {
            answers.push(next);
        }
    }
    answers
}
