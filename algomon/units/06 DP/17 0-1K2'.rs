use std::cmp;
use std::error;
use std::io;
use std::str::FromStr;

fn knapsack(weights: Vec<i32>, values: Vec<i32>, max_weight: i32) -> i32 {
    let mut table: Vec<i32> = vec![-1; (max_weight as usize) + 1];

    table[0] = 0; //base case

    for i in 0..weights.len() {
        let mut j: i32 = max_weight;
        while j >= weights[i] {
            if table[(j - weights[i]) as usize] != -1 {
                table[j as usize] = cmp::max(
                    table[j as usize],
                    table[(j - weights[i]) as usize] + values[i],
                );
            }

            j -= 1;
        }
    }
    if let Some(num) = table.iter().max() {
        return *num;
    } else {
        return 0;
    }
}
