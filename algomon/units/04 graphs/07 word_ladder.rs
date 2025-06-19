use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::io;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Combination {
    digits: Vec<i32>,
    steps: i32,
}

fn build_Combination(digits: Vec<i32>, steps: i32) -> Combination {
    Combination {
        digits: digits.clone(),
        steps: steps,
    }
}

fn get_neighbors(comb: &Combination) -> Vec<Combination> {
    let mut neighbors: Vec<Combination> = vec![];
    //up and down in sets of 2 by L to R digit place in the combination
    for i in 0..4 {
        let mut next_up = comb.digits.clone();
        next_up[i] = (next_up[i] + 1) % 10;

        let mut next_down = comb.digits.clone();
        next_down[i] = (next_down[i] + 9) % 10; // handles wrap-around for -1

        let distance = comb.steps + 1;

        neighbors.push(build_Combination(next_up, distance));
        neighbors.push(build_Combination(next_down, distance));
    }

    neighbors
}

fn word_ladder(begin: String, end: String, word_list: Vec<String>) -> i32 {
    // WRITE YOUR BRILLIANT CODE HERE
    0
}
