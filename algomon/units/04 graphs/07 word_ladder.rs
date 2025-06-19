use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::io;

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

fn word_ladder(begin: String, end: String, word_list: Vec<String>) -> i32 {
    let mut queue: VecDeque<Word> = VecDeque::new();
    let mut visited: HashSet<Vec<char>> = HashSet::new();

    let start: Vec<char> = begin.chars().collect();
    let target: Vec<char> = end.chars().collect();

    queue.push_back(build_Word(start.clone(), 0));

    while !queue.is_empty() {
        if let Some(word) = queue.pop_front() {
            if visited.insert(word.chars.clone()) {
                if word.chars == target {
                    return word.steps;
                }
                let neighbors = get_neighbors(&word, &word_list);
                for i in neighbors {
                    queue.push_back(i);
                }
            }
        }
    }
    0
}
