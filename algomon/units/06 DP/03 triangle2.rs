use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let m = triangle.len();
    let n = triangle[m - 1].len();
    let mut table = vec![vec![i32::MAX; n]; m];

    for i in 0..n {
        table[m - 1][i] = triangle[m - 1][i];
    }

    let mut i: usize = m - 2;
    while i > 0 {
        for j in 0..triangle[i].len() {
            table[i][j] = cmp::min(table[i + 1][j], table[i + 1][j + 1]) + triangle[i][j];
        }
        i -= 1;
    }

    table[0][0] = cmp::min(table[1][0], table[1][1]) + triangle[0][0];
    // println!("table: {:?}", table);
    table[0][0]
}
