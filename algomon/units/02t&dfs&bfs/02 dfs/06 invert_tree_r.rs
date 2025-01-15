//AM

fn invert_binary_tree(tree: Tree<i32>) -> Tree<i32> {
    tree.map(|node| {
        let Node { val, left, right } = *node;
        Box::new(Node {
            val: val,
            left: invert_binary_tree(right),
            right: invert_binary_tree(left),
        })
    })
}
