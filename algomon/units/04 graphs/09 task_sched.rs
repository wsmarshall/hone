use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

//on Rust Playground since unavailable on AlgoMon in Rust
fn main() {
    let tasks = vec!['a', 'b', 'c', 'd'];
    let requirements = vec![vec!['a', 'b'], vec!['c', 'b'], vec!['b', 'd']];

    let mut answer = vec![];
    let mut visited: HashSet<char> = HashSet::new();
    let mut queue: VecDeque<char> = VecDeque::new();

    //first entry is the node, second is the in-edge count
    let mut graph: HashMap<char, i32> = HashMap::new();

    //insert all nodes into the HashMap
    for i in tasks {
        graph.insert(i, 0);
    }

    //count up and record each node's in-degree
    for i in &requirements {
        *graph.entry(i[1]).or_insert(0) += 1;
    }

    //initialize the queue
    for (i, count) in graph.iter() {
        if count == &0 {
            visited.insert(*i);
            queue.push_back(*i);
        }
    }

    //do the topo sort
    while !queue.is_empty() {
        if let Some(c) = queue.pop_front() {
            if graph.get(&c) == Some(&0) {
                answer.push(c);

                for i in &requirements {
                    if i[0] == c {
                        *graph.entry(i[1]).or_insert(0) -= 1;
                    }
                    if graph.get(&i[1]) == Some(&0) && visited.insert(i[1]) {
                        queue.push_back(i[1]);
                    }
                }
            }
        }
    }

    let target = vec!['a', 'c', 'b', 'd'];
    println!("result: {:?}", answer);
    println!("should be: {:?}", target);
}
