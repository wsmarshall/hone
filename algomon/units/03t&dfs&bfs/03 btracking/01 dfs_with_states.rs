use std::error;
use std::io;
use std::str::FromStr;

struct Node<T> {
    pub val: T,
    pub children: Vec<Node<T>>,
}

fn dfs(root: Node<i32>, path: &mut Vec<String>, res: &mut Vec<String>) {
    if root.children.len() == 0 {
        path.push(root.val.to_string());
        res.push(path.join("->"));
        path.pop();
        return;
    }

    for i in root.children {
        path.push(root.val.to_string());
        dfs(i, path, res);
        path.pop();
    }
}

fn ternary_tree_paths(root: Node<i32>) -> Vec<String> {
    let mut path = Vec::<String>::new();
    let mut res = Vec::<String>::new();

    dfs(root, &mut path, &mut res);

    res
}
