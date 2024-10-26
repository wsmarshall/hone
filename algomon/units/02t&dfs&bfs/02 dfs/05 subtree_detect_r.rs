fn subtree_of_another_tree(root: Tree<i32>, sub_root: Tree<i32>) -> bool {
    //TODO
}

fn is_same_tree(tree1: Tree<i32>, tree2: Tree<i32>) -> bool {
    if root.is_none() && sub_root.is_none() {
        return true;
    }

    if root.is_none() || sub_root.is_none() {
        return false;
    } else if let Some(node1) = root {
        if let Some(node2) = sub_root {
            if node1.val != node2.val {
                return false;
            } else {
                return subtree_of_another_tree(Some(node1), node2.left)
                    || subtree_of_another_tree(Some(node1), node2.right);
            }
        }
    }
    false
}
