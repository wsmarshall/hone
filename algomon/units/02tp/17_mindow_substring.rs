use std::error;
use std::io;

const size: usize = 58; //number of characters in ascii 'A' -> 'z'
//NB middle has non-alpha chars

//checks to see if a slice contains another wrt character counts
fn contains_helper(original: &[u8], window: &[u8]) -> bool {
    for i, el in window.iter().enumerate() {
        if el > 0 {
            if original[i] != el {
                return false;
            }
        }
    }
    true
}

fn get_minimum_window(original: String, check: String) -> String {
    let og_bytes = original.as_bytes();
    let check_bytes = check.as_bytes();
    
    let idx = |c: u8| {(c - 'A' as u8) as usize};
    //TODO use clone_from_slice or copy_from_slice methods later
    let len = original.len();
    
    let mut og = [0u8; size];
    for i in 0..len {
        
    }
    
    let mut window = [0u8; size];
    let mut left = 0;
    
    for right in 0..len {
        window[idx(original.chars().nth(right))] += 1;
        while 
    }
    let mut ans: String;
    
    ans
}
