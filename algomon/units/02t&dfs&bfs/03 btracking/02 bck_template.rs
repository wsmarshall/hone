fn bck(n: i32, tokens: &Vec<&str>, res: &mut Vec<String>, path: &mut Vec<String>) {
    if path.len() >= n.try_into().unwrap() {
        res.push(path.join(""));
        return;
    }

    for i in tokens {
        path.push(i.clone().to_string());
        bck(n, tokens, res, path);
        path.pop();
    }
}

fn letter_combination(n: i32) -> Vec<String> {
    let tokens = vec!["a", "b"];
    let mut res = Vec::<String>::new();
    let mut path = Vec::<String>::new();

    bck(n, &tokens, &mut res, &mut path);

    res
}
