use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn maximal_square(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut table: Vec<Vec<i32>> = vec![vec![0; n]; m];
    let mut max: i32 = 0; //max length of the biggest square in the matrix

    if matrix[0][0] == 1 {
        table[0][0] = 1;
        max = 1;
    }
    for i in 1..m {
        if matrix[i][0] == 1 {
            table[i][0] = 1;
            max = cmp::max(max, 1);
        }
    }
    for i in 1..n {
        if matrix[0][i] == 1 {
            table[0][i] = 1;
            max = cmp::max(max, 1);
        }
    }

    for i in 1..m {
        for j in 1..n {
            if matrix[i][j] == 1 {
                let current = cmp::min(
                    table[i - 1][j],
                    cmp::min(table[i][j - 1], table[i - 1][j - 1]),
                ) + 1;
                max = cmp::max(max, current);
                table[i][j] = current;
            }
        }
    }

    max * max
}
