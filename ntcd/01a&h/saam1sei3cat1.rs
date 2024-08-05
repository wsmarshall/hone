impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        nums.iter().for_each(|&i| *map.entry(i).or_insert(0) += 1);
        let mut sorted = vec![Vec::new(); nums.len()];
        for (&k, &v) in map.iter() {
            sorted[nums.len() - v].push(k);
        }
        sorted.into_iter().flatten().take(k as usize).collect()
    }
}
