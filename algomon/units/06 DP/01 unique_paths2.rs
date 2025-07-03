use std::error;
use std::io;

//"1D array approach"
fn unique_paths(m: i32, n: i32) -> i32 {
    let r = m as usize; //row
    let c = n as usize; //col

    let mut current_row: Vec<i32> = vec![1; c];
    let mut prev_row: Vec<i32> = vec![1; c];

    for i in 1..r {
        for j in 1..c {
            current_row[j] = current_row[j - 1] + prev_row[j];
        }
        let temp = current_row;
        current_row = prev_row;
        prev_row = temp;
    }

    prev_row[c - 1]
}
