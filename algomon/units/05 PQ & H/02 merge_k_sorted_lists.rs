use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

struct Item {
    val: i32,
    list: usize,
    list_index: usize,
}

fn build_Item(value: i32, list: usize, list_index: usize) {
    Item {
        val: value,
        list: list,
        list_index: list_index,
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

fn merge_k_sorted_lists(lists: Vec<Vec<i32>>) -> Vec<i32> {
    // WRITE YOUR BRILLIANT CODE HERE
    Vec::new()
}
