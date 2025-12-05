use std::error;
use std::io;

fn unique_paths(m: i32, n: i32) -> i32 {
    let mut ans: i32 = 1;
    let new_n = m + n - 2;
    let k = m - 1;
    for i in 1..=k {
        ans = ans * (new_n + 1 - i) / i;
    }
    ans
}
