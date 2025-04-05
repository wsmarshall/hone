use std::error;
use std::io;
use std::str::FromStr;
use std::collections::HashSet;

fn least_consecutive_cards_to_match(cards: Vec<i32>) -> i32 {
    let length = cards.len();
    if length == 0 {
        return 0;
    }
    
    let mut hand = HashSet::new();
    let mut left = 0;
    let mut least_cards = (length + 1) as i32
    for right in 0..length {
        
        
    }
    
    least_cards
}