fn build_btree_r(
    preorder: &Vec<i32>,
    preordex: usize,
    inordex_start: usize,
    size: usize,
    val_to_index: &HashMap<i32, usize>,
) -> Tree<i32> {
    if size <= 0 {
        return None;
    }
    let root_val = preorder[preordex];

    let inorder_root_index = val_to_index[&root_val];

    let l_subtree_size = inorder_root_index - inordex_start;

    let left = build_btree_r(
        preorder,
        preordex + 1,
        inordex_start,
        l_subtree_size,
        &val_to_index,
    );
    let right = build_btree_r(
        preorder,
        preordex + 1 + l_subtree_size,
        inorder_root_index + 1,
        size - 1 - l_subtree_size,
        &val_to_index,
    );

    Some(Box::new(Node {
        val: root_val,
        left: left,
        right: right,
    }))
}

fn construct_binary_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Tree<i32> {
    let inordex: HashMap<i32, usize> = inorder
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<_, _>>();

    build_btree_r(&preorder, 0, 0, preorder.len(), &inordex)
}
