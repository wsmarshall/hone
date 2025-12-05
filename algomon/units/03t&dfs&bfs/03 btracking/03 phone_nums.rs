use std::collections::HashMap;
use std::error;
use std::fmt::Display;
use std::io;

fn dfs(
    start_index: usize,
    length: usize,
    number: &Vec<&str>,
    map: &HashMap<&str, Vec<&str>>,
    path: &mut Vec<String>,
    results: &mut Vec<String>,
) {
    if start_index >= length {
        results.push(path.join(""));
        return;
    }
    if let Some(v) = map.get(number[start_index]) {
        for i in v {
            path.push(i.to_string());
            dfs(start_index + 1, length, number, map, path, results);
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

    let length = number.len();
    let mut results: Vec<String> = vec![];
    let mut path: Vec<String> = vec![];

    dfs(0, length, &number, &map, &mut path, &mut results);

    results
}
