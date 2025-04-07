use std::collections::HashMap;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn subarray_sum(arr: Vec<i32>, target: i32) -> Vec<i32> {
    //holds prefix sum, index
    let mut pre_sums: HashMap<i32, usize> = HashMap::new();
    let mut pre_sum: i32 = 0;
    let mut indices = Vec::<i32>::new();

    pre_sums.insert(0, 0);

    for (i, el) in arr.iter().enumerate() {
        pre_sum += el;
        let complement = pre_sum - target;

        if let Some(n) = pre_sums.get(&complement) {
            indices.push((*n).try_into().unwrap());
            indices.push(i as i32 + 1);
            break;
        }
        pre_sums.insert(pre_sum.try_into().unwrap(), i + 1);
    }

    indices
}
