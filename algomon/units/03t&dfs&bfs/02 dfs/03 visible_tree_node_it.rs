fn visible_tree_node(root: Tree<i32>) -> i32 {
    let mut visible_nodes = 0;

    if root.is_none() {
        return 0;
    }

    if let Some(node) = root {
        let mut max = node.val;
        let mut stack = vec![(Some(node), max)];

        while let Some((Some(n), m)) = stack.pop() {
            if n.val >= m {
                visible_nodes += 1;
            }

            max = std::cmp::max(n.val, m);

            if n.right.is_some() {
                stack.push((n.right, max));
            }
            if n.left.is_some() {
                stack.push((n.left, max));
            }
        }
    }

    visible_nodes
}
