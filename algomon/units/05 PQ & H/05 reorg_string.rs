use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
struct Item {
    character: char,
    val: usize,
}

fn build_Item(character: char, value: usize) -> Item {
    Item {
        character: character,
        val: value,
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

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let n = s.len();
        let mut counts: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<Item> = BinaryHeap::new();
        for (k, v) in &counts {
            heap.push(build_Item(*k, *v as usize));
        }

        if let Some(max) = heap.peek() {
            if max.val > (n + 1) / 2 {
                return "".to_string();
            }
        }

        let mut collect: Vec<char> = Vec::with_capacity(n);
        let mut start = 0;
        while !heap.is_empty() {
            if let Some(entry) = heap.pop() {
                let mut count = 0;
                while count < entry.val {
                    collect.insert(start, entry.character);
                    count += 1;
                    start += 2;
                    if start > n {
                        start = 1;
                    }
                }
            }
        }

        let ans: String = collect.iter().collect();
        ans
    }
}
