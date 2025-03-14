use std::collections::HashMap;
use std::error;
use std::fmt::Display;
use std::io;

fn dfs(
    n: usize,
    start_index: usize,
    path: &mut Vec<String>,
    map: HashMap<&str, Vec<&str>>,
    number: Vec<&str>,
    res: &mut Vec<String>,
) {
    //is leaf
    if start_index >= n {
        res.push(path.join(""));
        return;
    }

    //get edges
    if let Some(v) = map.get(number[start_index]) {
        for i in v {
            path.push(i.to_string());
            dfs(n, start_index + 1, path, map.clone(), number.clone(), res);
            path.pop();
        }
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

    let mut number: Vec<&str> = digits.split("").collect();
    number.remove(0);
    number.pop();

    let mut res = Vec::<String>::new();
    let mut path = Vec::<String>::new();

    let n = number.len();
    dfs(n, 0, &mut path, map.clone(), number.clone(), &mut res);

    res
}
