use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::error;
use std::io;

fn nth_ugly_number(n: i32) -> i32 {
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    min_heap.push(Reverse(1));
    let mut seen: HashSet<i32> = HashSet::new();

    let mut count = 1;

    while count < n {
        if let Some(Reverse(num)) = min_heap.pop() {
            let first = num * 2;
            if seen.insert(first) {
                min_heap.push(Reverse(first));
            }

            let second = num * 3;
            if seen.insert(second) {
                min_heap.push(Reverse(second));
            }
            let third = num * 5;
            if seen.insert(third) {
                min_heap.push(Reverse(third));
            }
            count += 1;
        }
        //         println!("state of the heap: {:?}", min_heap);
    }

    if let Some(Reverse(n)) = min_heap.pop() {
        return n;
    }

    0
}
