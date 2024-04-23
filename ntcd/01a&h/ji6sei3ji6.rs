use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if (s.chars().count() != t.chars().count()) {
            return false;
        }

        let mut s_map = HashMap::<char, i32>::new();
        let mut t_map = HashMap::<char, i32>::new();

        for char in s.chars() {
            if (s_map.contains_key(&char)) {
                let new_count = add_one(s_map.get(&char));
                s_map.insert(char, new_count);
            } else {
                s_map.insert(char, 1);
            }
        }

        for char in t.chars() {
            if (t_map.contains_key(&char)) {
                let new_count = add_one(t_map.get(&char));
                t_map.insert(char, new_count);
            } else {
                t_map.insert(char, 1);
            }
        }

        return s_map == t_map;
    }
}

fn add_one(x: Option<&i32>) -> i32 {
    *x.unwrap_or(&0) + 1
}
