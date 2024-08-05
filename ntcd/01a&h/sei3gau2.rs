use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for word in strs {
            let mut freq: [u8; 26] = [0; 26];
            for byte in word.bytes() {
                freq[(byte - b'a') as usize] += 1;
            }
            if let Some(words) = result.get_mut(&freq) {
                words.push(word);
            } else {
                result.insert(freq, vec![word]);
            }
        }
        result.into_values().collect()
    }
}
