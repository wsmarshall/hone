use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::error;
use std::io;
use std::io;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Word {
    chars: Vec<char>,
    steps: i32,
}

fn build_Word(chars: Vec<char>, steps: i32) -> Word {
    Word {
        chars: chars.clone(),
        steps: steps,
    }
}

//takes in a word, gives back a list of words from the dictionary
//that are one character away
fn get_neighbors(word: &Word, dict: &Vec<String>) -> Vec<Word> {
    let mut neighbors: Vec<Word> = vec![];

    let chars = word.chars.clone(); //clone?
    for i in dict {
        let mut diffs = 0;
        let temp: Vec<char> = i.chars().collect();
        for j in 0..temp.len() {
            if temp[j] != chars[j] {
                diffs += 1;
            }
        }
        if diffs == 1 {
            neighbors.push(build_Word(i.chars().collect(), word.steps + 1));
        }
    }
    neighbors
}

fn num_steps(init_pos: Vec<Vec<i32>>) -> i32 {
    // WRITE YOUR BRILLIANT CODE HERE
    0
}
