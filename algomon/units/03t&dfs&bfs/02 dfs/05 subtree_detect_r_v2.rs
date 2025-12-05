fn subtree_of_another_tree(root: Tree<i32>, sub_root: Tree<i32>) -> bool {
    subtree_of_another_tree_r(&root, &sub_root)
}

fn subtree_of_another_tree_r(root: &Tree<i32>, subroot: &Tree<i32>) -> bool {
    match (root, subroot) {
        (None, None) => true,
        (Some(node1), Some(node2)) => {
            let left = subtree_of_another_tree_r(&node1.left, subroot);
            let right = subtree_of_another_tree_r(&node1.right, subroot);

            return is_same(&root, &subroot) || left || right;
        }
        _ => false,
    }
}

fn is_same(tree1: &Tree<i32>, tree2: &Tree<i32>) -> bool {
    match (tree1, tree2) {
        (None, None) => true,
        (Some(node1), Some(node2)) => {
            return node1.val == node2.val
                && is_same(&node1.left, &node2.left)
                && is_same(&node1.right, &node2.right);
        }
        _ => false,
    }
}
