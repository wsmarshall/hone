use std::collections::BinaryHeap;
use std::error;
use std::io;
use std::str::FromStr;

//takes a Heap approach (see other for faster but slightly trickier method)

fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();

    for i in nums {
        heap.push(i);
    }

    for i in 0..k - 1 {
        heap.pop();
    }

    if let Some(num) = heap.pop() {
        return num;
    }

    0
}
