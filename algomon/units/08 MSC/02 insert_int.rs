use std::cmp;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn insert_interval(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut answer: Vec<Vec<i32>> = vec![];
    let mut new_used = false;

    for i in intervals {
        if !new_used {
            if i[1] <= new_interval[0] {
                answer.push(i);
            } else if i[0] < new_interval[1] || i[1] > new_interval[0] {
                answer.push(vec![
                    cmp::min(i[0], new_interval[0]),
                    cmp::max(i[1], new_interval[1]),
                ]);
                new_used = true;
            }
        } else {
            answer.push(i);
        }
    }

    answer
}
