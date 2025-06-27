use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error;
use std::io;
use std::str::FromStr;

//nb 101 vs 102 test fails on AM (THEY'RE WRONG!)

fn assess_indegrees<'a>(reqs: &'a Vec<Vec<String>>, map: &mut HashMap<&'a String, i32>) {
    for i in reqs {
        *map.entry(&i[0]).or_insert(0);
        for j in 1..i.len() {
            *map.entry(&i[j]).or_insert(0) += 1;
        }
    }
}

fn topo_sort(
    requirements: &Vec<Vec<String>>,
    map: HashMap<&String, i32>,
    edge_map: HashMap<&String, HashSet<&String>>,
) -> i32 {
    let mut queue: VecDeque<&String> = VecDeque::new();
    let mut indegrees: HashMap<&String, i32> = HashMap::new();
    let mut total_time: i32 = 0;
    let mut visited: HashSet<&String> = HashSet::new();

    assess_indegrees(&requirements, &mut indegrees);
    // println!("indegrees: {:?}", indegrees);

    for i in &indegrees {
        if *i.1 == 0 {
            queue.push_back(*i.0);
        }
    }
    // println!("line 32 queue: {:?}", queue);

    let mut max_time = 0;
    for i in &queue {
        visited.insert(i);
        let curr_time = *(map.get(i)).unwrap();
        if curr_time > max_time {
            max_time = curr_time;
        }
    }

    total_time += max_time;
    // println!("44 visited: {:?}, total time: {}", visited, total_time);

    //     println!("indegrees: {:?}", indegrees);

    while !queue.is_empty() {
        if let Some(entry) = queue.pop_front() {
            //             println!("recon: {:?}", recon);
            //             println!("map: {:?}", map);
            //access indegrees, decrement
            if edge_map.contains_key(&entry) {
                let neighbors = edge_map.get(&entry).unwrap();
                for i in neighbors {
                    *indegrees.entry(i).or_insert(0) -= 1;
                }
                //find any now 0 nodes and add them to queue
                //                 println!("indegrees: {:?}", indegrees);
                for i in &indegrees {
                    if *i.1 == 0 && !visited.contains(i.0) {
                        queue.push_back(*i.0);
                    }
                }
            }
        }

        max_time = 0;
        for i in &queue {
            if visited.insert(i) {
                let curr_time = *(map.get(i)).unwrap();
                if curr_time > max_time {
                    max_time = curr_time;
                }
            }
        }
        total_time += max_time;
    }
    total_time
}

fn task_scheduling_2(tasks: Vec<String>, times: Vec<i32>, requirements: Vec<Vec<String>>) -> i32 {
    let mut map: HashMap<&String, i32> = HashMap::new(); //maps tasks to times
    for i in 0..tasks.len() {
        *map.entry(&tasks[i]).or_insert(0) = times[i];
    }

    let mut edge_map: HashMap<&String, HashSet<&String>> = HashMap::new();
    for i in &requirements {
        for j in 0..i.len() - 1 {
            // println!("first: {:?}, second: {:?}", &i[j], &i[j+1]);
            (*edge_map.entry(&i[j]).or_insert(HashSet::new())).insert(&i[j + 1]);
        }
    }
    // println!("requirements: {:?}", requirements);
    // println!("map: {:?}", map);
    //     println!("edge_map: {:?}", edge_map);

    topo_sort(&requirements, map, edge_map)
}
