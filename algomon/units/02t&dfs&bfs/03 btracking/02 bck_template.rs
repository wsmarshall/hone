fn bck(n: i32, tokens: &Vec<&str>, res: &mut Vec<&str>, path: &mut Vec<&str>) {
    if path.len() >= n.try_into().unwrap() {
        let word = path.join("");
        path.pop();
        res.push(word);
        return;
    }

    for i in tokens {
        path.push(i.clone());
        bck(n, tokens, res, path);
    }
}

fn letter_combination(n: i32) -> Vec<String> {
    let tokens = vec!["a", "b"];
    let mut res = Vec::<String>::new();
    let mut path = Vec::<String>::new();

    bck(n, &tokens, &mut res, &mut path);

    res
}
