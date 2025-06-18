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

fn get_neighbors(comb: &Combination, grid: &Vec<Vec<i32>>) -> Vec<Combination> {
    let mut neighbors: Vec<Combination> = vec![];
    //up and down in sets of 2 by L to R digit place in the combination
    for i in 0..4 {
        let mut next_up = comb.digits.clone();
        next_up[i] = match next_up[i] + 1 {
            10 => 0,
            _ => (i + 1).try_into().unwrap(),
        };

        let mut next_down = comb.digits.clone();
        next_down[i] = match next_up[i] - 1 {
            -1 => 9,
            _ => (i - 1).try_into().unwrap(),
        };

        let distance = comb.steps + 1;

        neighbors.push(build_Combination(next_up, distance));
        neighbors.push(build_Combination(next_down, distance));
    }

    neighbors
}

fn num_steps(target_combo: String, trapped_combos: Vec<String>) -> i32 {
    // use bfs
    let mut visited: HashSet<Combination> = HashSet::new();
    let mut queue: VecDeque<Combination> = VecDeque::new();

    let mut target: Vec<&str> = target_combo.split("").collect();
    target.remove(0);
    target.pop();
    let target = target
        .into_iter()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    0
}
