use std::error;
use std::io;

const size: usize = 58; //number of characters in ascii 'A' -> 'z'
                        //NB middle has non-alpha chars

//checks to see if a slice contains another wrt character counts
fn contains_helper(big: &[u8], window: &[u8]) -> bool {
    for (i, el) in window.iter().enumerate() {
        if el > 0 {
            if big[i] != el {
                return false;
            }
        }
    }
    true
}

fn get_minimum_window(original: String, check: String) -> String {
    let mut ans: String = String::new();
    let og_bytes = original.as_bytes();
    let check_bytes = check.as_bytes();

    let idx = |c: u8| (c - 'A' as u8) as usize;
    //TODO use clone_from_slice or copy_from_slice methods later

    //get the character counts from the 'check' string
    let mut check_count = [0u8; size];
    for i in 0..check.len() {
        check_count[idx(check.chars().nth(i))] += 1;
    }

    let mut window = [0u8; size];
    let mut min_len = original.len() + 1;
    let mut left = 0;

    for right in 0..original.len() {
        window[idx(original.chars().nth(right))] += 1;
        while contains_helper(&window, &check_count) {
            //check length, get new mindow if applicable
            if right - left < min_len {
                ans = &window[left..right + 1].copy_from_slice();
            } else if right - left == min_len {
                //whichever string is lexicographically first
                let s1 = ans.make_ascii_lowercase();
                let s2 = &window[left..right + 1]
                    .copy_from_slice()
                    .make_ascii_lowercase();
                match s1.cmp(&s2) {
                    std::cmp::Ordering::Greater => ans = &window[left..right + 1].copy_from_slice(),
                }
            }
            window[idx(original.chars().nth(left))] -= 1;
            left += 1;
        }
    }
    ans
}


/**
 * use std::collections::HashMap;

impl Solution 
{
    pub fn min_window(s: String, t: String) -> String 
    {
        let ss = s.as_bytes();
        
        // [1] obtain letter frequencies
        let mut freq: HashMap<u8,i32> = t
            .bytes()
            .fold(HashMap::new(), |mut f,ch| { *f.entry(ch).or_default() += 1; f });
        
        // [2] in this solution, we use a two-pointer sliding window 
        //    approach while keeping track of a number of unique 
        //    characters from 't' that are missing on each iteration 
        let mut miss      = freq.len() as i32; 
        let (mut l, mut r)  = (0, 0);
        let (mut wl, mut wr) = (0, s.len()+1);
        
        loop
        {
            // [3] move right pointer while we're still 
            //    missing any of the characters from 't' 
            if r < ss.len() && miss > 0
            {
                if let Some(f) = freq.get_mut(&ss[r])
                {
                    *f -= 1;
                    if *f == 0 { miss -= 1; }
                }
                r += 1;
            }
                
            // [4] move left pointer while we're still 
            //    having a surplus of characters from 't' 
            else if l < ss.len() && miss <= 0
            {
                // [5] the update of minimal substring happens only when 
                //    we move the left pointer, i.e., when we already 
                //    have no characters from 't' missing 
                if r-l < wr-wl { wl = l; wr = r; }

                if let Some(f) = freq.get_mut(&ss[l])
                {
                    if *f == 0 { miss += 1; }
                    *f += 1;
                }
                l += 1;
            }
            else { break; }
        }
        
        return if wr <= ss.len() { s[wl..wr].to_string() } else { String::new() };
    }
}
 */