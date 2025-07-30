use std::error;
use std::io;

fn perfect_squares(n: i32) -> i32 {
    let mut total: usize = n.try_into().unwrap();
    let mut count: i32 = 0;
    let root: usize = ((n as f64).sqrt().floor()) as usize;

    let mut i = root;
    while i > 0 {
        // println!("i: {}", i);
        let current = i * i;
        while (total as i32 - current as i32) >= 0 {
            count += 1;
            // println!("total: {}", total);
            total -= current;
        }
        i -= 1;
    }

    count
}
