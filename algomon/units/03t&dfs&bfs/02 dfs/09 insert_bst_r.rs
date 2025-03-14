fn insert_bst(bst: Tree<i32>, val: i32) -> Tree<i32> {
    match bst {
        None => {
            return Some(Box::new(Node {
                val: val,
                left: None,
                right: None,
            }))
        }
        Some(mut node) => {
            if node.val < val {
                node.right = insert_bst(node.right, val);
            } else if node.val > val {
                node.left = insert_bst(node.left, val);
            }

            return Some(node);
        }
    };
}
