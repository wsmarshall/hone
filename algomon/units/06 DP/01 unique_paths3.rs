use std::error;
use std::io;
//WHY IS THIS???

fn unique_paths(m: i32, n: i32) -> i32 {
    let mut ans: i32 = 1;
    for i in 1..m {
        ans = ans * (n - 1 + i) / i;
    }
    ans
}
