fn visible_tree_node(root: Tree<i32>) -> i32 {
    if let Some(n) = root {
        let max = n.val;
        return visible_tree_node_helper(Some(n), max);
    }
    0
}

fn visible_tree_node_helper(node: Tree<i32>, max: i32) -> i32 {
    if let Some(n) = node {
        let is_visible: i32;
        if n.val >= max {
            is_visible = 1;
        } else {
            is_visible = 0;
        }
        let max = std::cmp::max(n.val, max);

        return is_visible
            + (visible_tree_node_helper(n.left, max) + visible_tree_node_helper(n.right, max));
    }
    0
}
