use std::collections::HashMap;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn subarray_sum(arr: Vec<i32>, target: i32) -> Vec<i32> {
    //holds prefix sum, index
    let mut pre_sums = HashMap::new();
    let mut sum = 0;
    let mut indices = Vec::<i32>::new();

    Vec::new()
}
