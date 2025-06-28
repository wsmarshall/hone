use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

fn distance(point: &Vec<i32>) -> i32 {
    let x = point[0];
    let y = point[1];
    let dist = ((x * x + y * y) as f64).sqrt() as i32;

    dist
}

fn k_closest_points(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut p_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    //maps distances to Vec of a point's coordinates
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut ans: Vec<Vec<i32>> = vec![];

    for i in points {
        let d = distance(&i);
        p_heap.push(Reverse(d));
        map.insert(d, i.clone());
    }

    for _i in 0..k {
        if let Some(p) = p_heap.pop() {
            ans.push(map.get(&(p.0)).unwrap().to_vec());
        }
    }

    ans
}
