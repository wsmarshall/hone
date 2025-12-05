use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

//this assumes any input has only 2 i32 entries
//note that this return value should always be positive
fn distance(point: &Vec<i32>) -> i32 {
    let x = point[0];
    let y = point[1];
    ((x * x) + (y * y))
}

fn k_closest_points(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut min_heap: BinaryHeap<Reverse<(i32, Vec<i32>)>> = BinaryHeap::new();
    let mut ans: Vec<Vec<i32>> = vec![];
    for p in points {
        let d: i32 = distance(&p);
        min_heap.push(Reverse((d, p.clone())));
    }

    for _i in 0..k {
        ans.push(min_heap.pop().unwrap().0 .1.to_vec());
    }

    ans
}
