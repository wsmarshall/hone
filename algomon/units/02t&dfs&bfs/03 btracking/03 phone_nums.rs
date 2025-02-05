use std::collections::HashMap;
use std::error;
use std::fmt::Display;
use std::io;

fn dfs(
    n: usize,
    start_index: usize,
    path: &mut Vec<String>,
    map: HashMap<String, Vec<String>>,
    number: Vec<String>,
    res: &mut Vec<String>,
) {
    //is leaf
    if start_index >= n {
        res.push(path.collect());
        return;
    }

    //get edges
    for i in map.get(number[i]) {
        path.push(i);
        dfs(n, start_index + 1, path, map, number, res);
        path.pop();
    }
}

fn letter_combinations_of_phone_number(digits: String) -> Vec<String> {
    let map = HashMap::from([
        ("2", vec!["a", "b", "c"]),
        ("3", vec!["d", "e", "f"]),
        ("4", vec!["g", "h", "i"]),
        ("5", vec!["j", "k", "l"]),
        ("6", vec!["m", "n", "o"]),
        ("7", vec!["p", "q", "r", "s"]),
        ("8", vec!["t", "u", "v"]),
        ("9", vec!["w", "x", "y", "z"]),
    ]);

    let number: Vec<&str> = digits.split("").collect();

    let mut res = Vec::<String>::new();
    let mut path = Vec::<String>::new();

    let n = number.len();

    dfs(n, 0, &mut path, map, number, &mut res);

    res
}
