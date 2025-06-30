use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error;
use std::io;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Item {
    val: i32,
    row: usize,
    col: usize,
    row_len: usize,
}

fn build_Item(value: i32, row: usize, col: usize, row_len: usize) -> Item {
    Item {
        val: value,
        row: row,
        col: col,
        row_len: row_len,
    }
}

impl PartialOrd<Item> for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        other.val.partial_cmp(&self.val) // - for MIN HEAP
                                         //self.val.partial_cmp(&other.val)   // - for MAX HEAP
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val) // - for MIN HEAP
                                 //self.val.cmp(&other.val)           // - for MAX HEAP
    }
}

fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    // WRITE YOUR BRILLIANT CODE HERE
    0
}
