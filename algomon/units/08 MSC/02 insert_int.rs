use std::cmp;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn insert_interval(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut answer: Vec<Vec<i32>> = vec![];
    let mut new_used = false;
    let mut new_pot = new_interval.clone();

    for i in intervals {
        if !new_used {
            if i[1] < new_pot[0] {
                answer.push(i);
            } else if i[1] >= new_pot[0] || i[0] <= new_pot[1] {
                new_pot = (vec![cmp::min(i[0], new_pot[0]), cmp::max(i[1], new_pot[1])]);
            } else if i[0] <= new_pot[1] && !new_used {
                answer.push(new_pot.clone());
                answer.push(i);
                new_used = true;
            }
        } else {
            answer.push(i);
        }
    }

    answer
}
