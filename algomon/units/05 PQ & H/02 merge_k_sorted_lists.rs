use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Item {
    val: i32,
    list: usize,
    list_len: usize,
    list_index: usize,
}

fn build_Item(value: i32, list: usize, list_len: usize, list_index: usize) -> Item {
    Item {
        val: value,
        list: list,
        list_len: list_len,
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
    let mut ans: Vec<i32> = vec![];
    let mut k_heap: BinaryHeap<Item> = BinaryHeap::new();

    //adds the minimal element from each list to the minheap
    for i in 0..lists.len() {
        let current = build_Item(lists[i][0], i, lists[i].len(), 0);
        k_heap.push(current);
    }

    while !k_heap.is_empty() {
        //         println!("heap: {:?}", k_heap);
        if let Some(current_item) = k_heap.pop() {
            ans.push(current_item.val);
            let next_index = current_item.list_index + 1;
            if next_index < lists[current_item.list].len() {
                let next_val = lists[current_item.list][next_index];
                k_heap.push(build_Item(
                    next_val,
                    current_item.list,
                    current_item.list_len,
                    next_index,
                ));
            }
        }
    }

    ans
}
