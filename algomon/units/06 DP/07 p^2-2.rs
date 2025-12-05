use std::cmp;
use std::error;
use std::io;

fn perfect_squares(n: i32) -> i32 {
    let length = n as usize;
    let mut table = vec![i32::MAX; length + 1];
    table[0] = 0;

    let mut i: usize = 1;
    let mut current: usize = i * i;
    let target: usize = n as usize;
    while current <= target {
        for j in current..=length {
            table[j] = cmp::min(table[j], table[j - current] + 1);
        }
        i += 1;
        current = i * i;
    }
    table[length]
}
