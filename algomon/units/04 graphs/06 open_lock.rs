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

fn num_steps(target_combo: String, trapped_combos: Vec<String>) -> i32 {
    // use bfs
    let mut visited: HashSet<Vec<i32>> = HashSet::new();
    let mut queue: VecDeque<Combination> = VecDeque::new();

    let mut target: Vec<&str> = target_combo.split("").collect();
    target.remove(0);
    target.pop();
    let target: Vec<i32> = target
        .into_iter()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut traps: HashSet<Vec<i32>> = HashSet::new();
    for i in trapped_combos {
        let mut current: Vec<_> = i.split("").collect();
        current.remove(0);
        current.pop();
        let current: Vec<i32> = current
            .into_iter()
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        traps.insert(current);
    }

    queue.push_back(build_Combination(vec![0, 0, 0, 0], 0));

    while !queue.is_empty() {
        if let Some(comb) = queue.pop_front() {
            if visited.insert(comb.digits.clone()) {
                if comb.digits == target {
                    return comb.steps;
                }
                let neighbors = get_neighbors(&comb);
                for i in neighbors {
                    if !traps.contains(&i.digits) {
                        queue.push_back(i);
                    }
                }
            }
        }
    }
    -1
}
