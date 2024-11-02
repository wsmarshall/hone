fn subtree_of_another_tree(root: Tree<i32>, sub_root: Tree<i32>) -> bool {
    if let Some(node) = root {
        if let Some(other_node) = sub_root {
            return is_same_tree(Some(node.clone()), Some(other_node.clone()))
                || subtree_of_another_tree(node.left, Some(other_node.clone()))
                || subtree_of_another_tree(node.right, Some(other_node.clone()));
        }
    }
    if root.is_none() && sub_root.is_none() {
        return true;
    }
    false
}

fn is_same_tree(tree1: Tree<i32>, tree2: Tree<i32>) -> bool {
    if tree1.is_none() && tree2.is_none() {
        return true;
    }

    if tree1.is_none() || tree2.is_none() {
        return false;
    } else if let Some(node1) = tree1 {
        if let Some(node2) = tree2 {
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
