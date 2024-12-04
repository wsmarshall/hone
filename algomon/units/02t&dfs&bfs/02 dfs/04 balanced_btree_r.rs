fn is_balanced(tree: Tree<i32>) -> bool {
    if tree.is_none() {
        return true;
    }

    return tree_height(tree) != -1;
}

fn tree_height(tree: Tree<i32>) -> i32 {
    if let Some(n) = tree {
        let left_height = tree_height(n.left);
        let right_height = tree_height(n.right);

        if left_height == -1 || right_height == -1 {
            return -1;
        }

        if (left_height - right_height).abs() > 1 {
            return -1;
        }

        return std::cmp::max(left_height, right_height) + 1;
    } else {
        //subtree is null
        return 0;
    }
}
