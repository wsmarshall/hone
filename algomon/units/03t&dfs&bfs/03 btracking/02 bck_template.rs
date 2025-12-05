fn bck(
    n: i32,
    start_index: usize,
    tokens: &Vec<&str>,
    res: &mut Vec<String>,
    path: &mut Vec<String>,
) {
    if start_index >= n.try_into().unwrap() {
        res.push(path.join(""));
        return;
    }

    for i in tokens {
        path.push(i.clone().to_string());
        bck(n, start_index + 1, tokens, res, path);
        path.pop();
    }
}

fn letter_combination(n: i32) -> Vec<String> {
    let tokens = vec!["a", "b"];
    let mut res = Vec::<String>::new();
    let mut path = Vec::<String>::new();

    bck(n, 0, &tokens, &mut res, &mut path);

    res
}
