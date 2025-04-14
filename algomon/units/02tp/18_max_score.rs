use std::error;
use std::io;
use std::str::FromStr;

fn maximum_score(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut e1 = 0;
    let mut e2 = 0;
    let n1 = arr1.len();
    let n2 = arr2.len();
    let mut sum1 = 0;
    let mut sum2 = 0;

    let mut max_score = 0;

    while e1 < n1 || e2 < n2 {
        if arr1[e1] == arr2[e2] {
            max_score += std::cmp::max(sum1, sum2);
        } else if arr1[e1] <= arr2[e2] || e1 < n1 {
            sum1 += arr1[e1];
            e1 += 1;
        } else {
            //arr1[e1] > arr2[e2]
            sum2 += arr2[e2];
            e2 += 1;
        }
    }

    max_score
}
