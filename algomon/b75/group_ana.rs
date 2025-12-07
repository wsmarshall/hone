use std::collections::HashMap;
use std::error;
use std::fmt::Display;
use std::io;

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut answers: Vec<Vec<String>> = Vec::new();

    for s in strs {
        let mut sorted: Vec<char> = s.chars().collect();
        sorted.sort();
        let key: String = sorted.iter().collect();

        if let Some(vector) = map.get_mut(&key) {
            vector.push(s.clone());
        } else {
            map.insert(key, vec![s.clone()]);
        }
    }

    for e in map {
        answers.push(e.1);
    }
    answers
}
