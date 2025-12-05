use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn heap_top_3(arr: Vec<i32>) -> Vec<i32> {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut ans: Vec<i32> = vec![];

    for i in arr {
        heap.push(Reverse(i));
    }

    for i in 0..3 {
        if let Some(num) = heap.pop() {
            ans.push(num.0);
        }
    }

    ans
}
