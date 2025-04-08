use std::collections::HashMap;
use std::error;
use std::io;
use std::str::FromStr;

fn subarray_sum_total(arr: Vec<i32>, target: i32) -> i32 {
    let length: i32 = arr.len().try_into().unwrap();
    if length == 0 {
        return 0;
    }

    let mut pre_sums: HashMap<i32, i32> = HashMap::new();
    pre_sums.insert(0, 1);

    let mut pre_sum: i32 = 0;
    let mut count = 0;
    for i in arr {
        pre_sum += i;
        let complement = pre_sum - target;

        if let Some(n) = pre_sums.get(&complement) {
            count += n;
        }

        *pre_sums.entry(pre_sum).or_insert(0) += 1;
    }
    count
}
