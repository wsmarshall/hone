use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::io;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    tiles: Vec<Vec<i32>>,
    steps: i32,
}

fn build_State(tiles: Vec<Vec<i32>>, steps: i32) -> State {
    State {
        tiles: tiles.clone(),
        steps: steps,
    }
}

//takes in a puzzle tile state, gives back a vec of states which
//are one tile move away
fn get_neighbors(state: &State) -> Vec<State> {
    let mut neighbors: Vec<State> = vec![];
    let mut zero_row: usize = 0;
    let mut zero_col: usize = 0;

    //finds the row and col of the zero
    for i in 0..2 {
        for j in 0..3 {
            if state.tiles[i][j] == 0 {
                zero_row = i;
                zero_col = j;
            }
        }
    }

    //swap left
    if zero_col == 1 || zero_col == 2 {
        let mut new_state = state.tiles.clone();
        let temp = new_state[zero_row][zero_col - 1];
        new_state[zero_row][zero_col - 1] = 0;
        new_state[zero_row][zero_col] = temp;
        neighbors.push(build_State(new_state, state.steps + 1));
    }

    //swap right
    if zero_col == 0 || zero_col == 1 {
        let mut new_state = state.tiles.clone();
        let temp = new_state[zero_row][zero_col + 1];
        new_state[zero_row][zero_col + 1] = 0;
        new_state[zero_row][zero_col] = temp;
        neighbors.push(build_State(new_state, state.steps + 1));
    }

    //swap up/down
    if zero_row == 0 {
        let mut new_state = state.tiles.clone();
        let temp = new_state[zero_row + 1][zero_col];
        new_state[zero_row + 1][zero_col] = 0;
        new_state[zero_row][zero_col] = temp;
        neighbors.push(build_State(new_state, state.steps + 1));
    } else {
        //zero_row is 1
        let mut new_state = state.tiles.clone();
        let temp = new_state[zero_row - 1][zero_col];
        new_state[zero_row - 1][zero_col] = 0;
        new_state[zero_row][zero_col] = temp;
        neighbors.push(build_State(new_state, state.steps + 1));
    }

    neighbors
}

fn num_steps(init_pos: Vec<Vec<i32>>) -> i32 {
    let end_state = vec![vec![1, 2, 3], vec![4, 5, 0]];
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut visited: HashSet<Vec<Vec<i32>>> = HashSet::new();

    queue.push_back(build_State(init_pos, 0));

    while !queue.is_empty() {
        if let Some(state) = queue.pop_front() {
            if visited.insert(state.tiles.clone()) {
                if state.tiles == end_state {
                    return state.steps;
                }
                let neighbors = get_neighbors(&state);
                for i in neighbors {
                    queue.push_back(i);
                }
            }
        }
    }
    -1
}
