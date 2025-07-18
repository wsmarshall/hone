use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

//small space optimization on previous
fn knapsack_weight_only(weights: Vec<i32>) -> Vec<i32> {
    let m: usize = weights.len();
    let n: usize = weights.iter().sum::<i32>() as usize;
    let mut current = vec![false; n + 1]; //sum validity, first table
    let mut prev = vec![false; n + 1]; //sum validity, second table

    prev[0] = true;
    for i in 1..=m {
        for w in 0..=n {
            current[w] = current[w] || prev[w];

            if w as i32 - weights[i - 1] >= 0 {
                current[w] = current[w] || prev[w - weights[i - 1] as usize];
            }
        }
        let temp = current;
        current = prev;
        prev = temp;
    }
    // println!("current: {:?}, prev: {:?}", current, prev);
    let mut answers = vec![];
    for i in 0..=n {
        if prev[i] {
            answers.push(i as i32);
        }
    }
    answers
}
