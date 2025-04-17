use std::error;
use std::io;

const size: usize = 58; //number of characters in ascii 'A' -> 'z'
                        //NB middle has non-alpha chars


//checks to see if a slice contains another
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
    let idx = |c: u8| (c - 'A' as u8) as usize;
    //TODO use clone_from_slice or copy_from_slice methods later
    let mut ans: String;

    ans
}
