use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
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

impl PartialOrd<Item> for Item 
{
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> 
    {
        other.val.partial_cmp(&self.val)     // - for MIN HEAP
        //self.val.partial_cmp(&other.val)   // - for MAX HEAP
    }
}

impl Ord for Item 
{
    fn cmp(&self, other: &Self) -> Ordering 
    {
        other.val.cmp(&self.val)             // - for MIN HEAP
        //self.val.cmp(&other.val)           // - for MAX HEAP
    }
}

fn merge_k_sorted_lists(lists: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];
    let mut k_heap: BinaryHeap<Item> = BinaryHeap::new();
    
    //these 2 variables are for knowing when we're done
    //with all the individual lists
    let mut total_num = 0;
    let mut current_count = 0;
    
    //adds the minimal element from each list to the minheap
    for i in 0..lists.len() {
        total_num += lists[i].len()-1;
        let current = build_Item(lists[i][0], i, lists[i].len(), 1);
        k_heap.push(current);
        current_count += 1;
    }
    
    while current_count <= total_num {
        let current_item = k_heap.pop();
        ans.push(current_item.val);
        
        
        
    Vec::new()
}