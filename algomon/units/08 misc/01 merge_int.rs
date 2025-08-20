use std::cmp;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn overlap(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    !(a[1] < b[0] || a[0] > b[1])
}

fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort();
    intervals.reverse();

    let mut answers: Vec<Vec<i32>> = vec![];

    for i in intervals {
        let len = answers.len();
        if answers.is_empty() || !overlap(&answers[len - 1], &i) {
            answers.push(i);
        } else {
            let mut last_interval = &mut answers[len - 1];
            last_interval[1] = cmp::max(last_interval[1], i[1]);
        }
    }

    answers
}
