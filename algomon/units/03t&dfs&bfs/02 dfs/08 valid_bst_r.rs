fn vcheck(node: &Tree<i32>, min: i32, max: i32) -> bool {
    let n = match node {
        None => return true,
        Some(thing) => thing,
    };

    if !(min <= n.val && n.val <= max) {
        return false;
    }

    vcheck(&n.left, min, n.val) && vcheck(&n.right, n.val, max)
}

fn valid_bst(root: Tree<i32>) -> bool {
    if root.is_none() {
        return true;
    }

    vcheck(&root, i32::MIN, i32::MAX)
}
