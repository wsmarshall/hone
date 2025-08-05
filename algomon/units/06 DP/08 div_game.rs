use std::error;
use std::io;

fn divisor_game(n: i32) -> bool {
    let n: usize = n.try_into().unwrap();
    let mut table: Vec<bool> = vec![false; n + 1];

    for i in 2..=n {
        for j in 1..i {
            if i % j == 0 && !table[i - j] {
                table[i] = true;
                break;
            }
        }
    }

    table[n]
}
