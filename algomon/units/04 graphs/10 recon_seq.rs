use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::io;
use std::str::FromStr;

fn assess_indegrees(graph: &mut HashMap<i32, i32>, seqs: &Vec<Vec<i32>>) {
    for i in seqs {
        graph.entry(i[0]).or_insert(0);
        for j in 1..i.len() {
            *graph.entry(i[j]).or_insert(0) += 1;
        }
    }
}

fn topo_sort(original: &Vec<i32>, seqs: &Vec<Vec<i32>>, map: HashMap<i32, HashSet<i32>>) -> bool {
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut indegrees: HashMap<i32, i32> = HashMap::new();
    let mut recon = vec![];

    assess_indegrees(&mut indegrees, &seqs);

    for i in indegrees {
        if indegrees.get(&i) == &0 {
            queue.push_back(i);
        }
    }

    while !queue.is_empty() {
        if queue.size() > 1 {
            return false; //no unique reconstruction
        }
        if let Some(num) = queue.pop_front() {
            //add num to the reconstruction
            recon.push(num);

            //decrement neighbors num points to

            //find any now 0 nodes and add them to queue
        }
    }

    original == recon
}

fn sequence_reconstruction(original: Vec<i32>, seqs: Vec<Vec<i32>>) -> bool {
    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for i in seqs {
        let first = i[0];
        for j in 1..i.len() {
            *map.entry(first).or_insert().insert(i[j]);
        }
    }

    topo_sort(&original, &seqs, map)
}
