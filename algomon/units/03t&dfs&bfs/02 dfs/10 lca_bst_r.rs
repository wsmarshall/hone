//recursive approach, faster than first pass self gen

fn lca_on_bst(bst: Tree<i32>, p: i32, q: i32) -> i32 {
    if let Some(node) = bst {
        if p > node.val && q > node.val {
            return lca_on_bst(node.right, p, q);
        }
        if p < node.val && q < node.val {
            return lca_on_bst(node.left, p, q);
        }
        return node.val;
    }
    0
}
