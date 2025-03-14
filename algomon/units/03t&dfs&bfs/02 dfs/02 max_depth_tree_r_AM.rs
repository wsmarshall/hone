fn tree_max_depth(root: Tree<i32>) -> i32 {
    if root.is_none() {
        return 0;
    }

    tree_max_depth_helper(root) - 1
}

fn tree_max_depth_helper(root: Tree<i32>) -> i32 {
    if let Some(node) = root {
        return std::cmp::max(
            tree_max_depth_helper(node.left),
            tree_max_depth_helper(node.right),
        ) + 1;
    }
    0
}
