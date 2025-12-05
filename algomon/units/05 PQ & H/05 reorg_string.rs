use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let n = s.len();
        let mut counts: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();
        for (k, v) in &counts {
            heap.push((*v as usize, *k));
        }

        let mut collect: Vec<char> = Vec::with_capacity(n);

        while heap.len() >= 2 {
            let (freq1, char1) = heap.pop().unwrap();
            let (freq2, char2) = heap.pop().unwrap();

            collect.push(char1);
            collect.push(char2);

            if freq1 > 1 {
                heap.push((freq1 - 1, char1));
            }
            if freq2 > 1 {
                heap.push((freq2 - 1, char2));
            }
        }

        if let Some((freq, ch)) = heap.pop() {
            if freq > 1 {
                return "".to_string();
            }
            collect.push(ch);
        }

        let ans: String = collect.iter().collect();
        ans
    }
}
