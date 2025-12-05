You are given a string s consisting of only uppercase english characters 
and an integer k. 
    You can choose up to k characters of the string and 
        replace them with any other uppercase English character.

After performing at most k replacements, 
    return the length of the longest substring which contains only one distinct character.

Example 1:
    Input: s = "XYYX", k = 2
    Output: 4
Explanation: Either replace the 'X's with 'Y's, or replace the 'Y's with 'X's.

Example 2:
    Input: s = "AAABABB", k = 1
Output: 5
Explanation: Replace first instance of 'B' with 'A' and output 5 'A' characters in sequence

Constraints:
1 <= s.length <= 1000
0 <= k <= s.length



use std::collections::HashMap;

impl Solution {
    let mut count_map = HashMap::new();

    //input is s
    let s_count = s.chars().collect();

    count_map.map(|s| s_count)* //go through Vec s_count to get count of distinct English characters

    let mut longest_count =0;
    for i in 0..s_count-1-k {
        let curr_char = s_count[i];
        let next_char = s_count[i+1];

        if curr_char == next_char {
            longest_count += 1;
        } else {
            if curr_char == s_count[i+k] {
                
            }
        }

    }








}