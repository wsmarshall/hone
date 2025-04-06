use std::collections::HashMap;
use std::error;
use std::io;
use std::str::FromStr;

fn least_consecutive_cards_to_match(cards: Vec<i32>) -> i32 {
    let length = cards.len();
    if length == 0 {
        return 0;
    }

    let mut hand = HashMap::new();
    let mut left = 0;
    let mut least_cards = (length + 1) as i32;
    for right in 0..length {
        while hand.contains_key(&cards[right]) {
            least_cards = std::cmp::min(least_cards, (right - left + 1) as i32);
            hand.remove(&cards[left]);
            left += 1;
        }
        *hand.entry(cards[right]).or_insert(0) += 1;
    }

    if least_cards > length.try_into().unwrap() {
        return -1;
    }
    least_cards
}
