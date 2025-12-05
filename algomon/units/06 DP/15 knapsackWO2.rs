use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn knapsack_weight_only(weights: Vec<i32>) -> Vec<i32> {
    let m: usize = weights.len();
    let n: usize = weights.iter().sum::<i32>() as usize;
    let mut table = vec![vec![false; n + 1]; m + 1]; //sum validity

    table[0][0] = true;
    for i in 1..=m {
        for w in 0..=n {
            table[i][w] = table[i][w] || table[i - 1][w];

            if w as i32 - weights[i - 1] >= 0 {
                table[i][w] = table[i][w] || table[i - 1][w - weights[i - 1] as usize];
            }
        }
    }
    // println!("table: {:?}", table);
    let mut answers = vec![];
    for i in 0..=n {
        if table[m][i] {
            answers.push(i as i32);
        }
    }
    answers
}
