use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::io;
use std::str::FromStr;

fn assess_indegrees(graph: &mut HashMap<i32, i32>, map: &HashMap<i32, HashSet<i32>>) {
    //     println!("map: {:?}", map);
    for (k, v) in map {
        *graph.entry(*k).or_insert(0);
        for j in v {
            *graph.entry(*j).or_insert(0) += 1;
        }
    }
}

fn topo_sort(original: &Vec<i32>, seqs: &Vec<Vec<i32>>, map: HashMap<i32, HashSet<i32>>) -> bool {
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut indegrees: HashMap<i32, i32> = HashMap::new();
    let mut recon = vec![];

    assess_indegrees(&mut indegrees, &map);

    for i in &indegrees {
        if *i.1 == 0 {
            queue.push_back(*i.0);
        }
    }

    //     println!("indegrees: {:?}", indegrees);

    while !queue.is_empty() {
        if queue.len() > 1 {
            return false; //no unique reconstruction
        }
        if let Some(num) = queue.pop_front() {
            //add num to the reconstruction
            recon.push(num);
            //             println!("recon: {:?}", recon);
            //             println!("map: {:?}", map);
            //access HashSet, decrement neighbors num points to
            if map.contains_key(&num) {
                let neighbors = map.get(&num).unwrap();
                //                 println!("neighbors: {:?}", neighbors);
                for i in neighbors.iter() {
                    *indegrees.entry(*i).or_insert(0) -= 1;
                }

                //find any now 0 nodes and add them to queue
                //                 println!("indegrees: {:?}", indegrees);
                for i in &indegrees {
                    if *i.1 == 0 {
                        if !recon.contains(i.0) {
                            //                             println!("new 0 entry: {:?}", i.0);
                            queue.push_back(*i.0);
                        }
                    }
                }
            }
        }
    }

    *original == recon
}

fn task_scheduling_2(tasks: Vec<String>, times: Vec<i32>, requirements: Vec<Vec<String>>) -> i32 {
    // WRITE YOUR BRILLIANT CODE HERE
    0
}
