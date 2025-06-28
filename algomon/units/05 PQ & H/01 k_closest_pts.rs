use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

#[derive(Eq, PartialEq)]
struct Point {
    x_y: Vec<i32>,
    dist: i32,
}

fn build_Point(coords: &Vec<i32>, distance: i32) -> Point {
    Point {
        x_y: coords.clone(),
        dist: distance,
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn distance(point: &Vec<i32>) -> i32 {
    let x = point[0];
    let y = point[1];
    let dist = ((x * x + y * y) as f64).sqrt() as i32;

    dist
}

fn k_closest_points(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let p_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut ans: Vec<Vec<i32>> = vec![];
    for i in points {
        let d = distance(&i);
        p_heap.push(Reverse(build_Point(&i, d)));
    }

    for i in 0..k {
        if let Some(p) = p_heap.pop() {
            ans.push(p.0.x_y);
        }
    }

    ans
}
