use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let m = triangle.len();
    let n = triangle[m - 1].len();
    let mut current: Vec<i32> = vec![0; n];
    let mut prev;

    for i in 0..n {
        current[i] = triangle[m - 1][i];
    }

    prev = current.clone();

    let mut i: usize = m - 2;
    while i > 0 {
        for j in 0..triangle[i].len() {
            current[j] = cmp::min(prev[j], prev[j + 1]) + triangle[i][j];
        }
        i -= 1;
        prev = current.clone();
    }

    current[0] = cmp::min(prev[0], prev[1]) + triangle[0][0];
    // println!("table: {:?}", table);
    current[0]
}
