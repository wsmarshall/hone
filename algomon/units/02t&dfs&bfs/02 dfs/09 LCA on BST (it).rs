fn lca_on_bst(bst: Tree<i32>, p: i32, q: i32) -> i32 {
    let mut curr_node = bst;
    while let Some(node) = curr_node {
        let lval;
        let rval;
        
        if node.right.is_some() {
            rval = node.right.as_ref().unwrap().val;
        } else {
            rval = node.val;
        }
        
        if node.left.is_some() {
            lval = node.left.as_ref().unwrap().val;
        } else {
            lval = node.val;
        }
        
        //check if current node is the LCA
        //NB this means smth like p <= curr_node <= q
        if (lval == p && rval == q) 
            || (lval == q && rval == p)
            || (node.val == p && (lval == q || rval == q))
            || (node.val == q && (lval == p || rval == p))
        {
            return node.val;
        }
        
        //since current node is NOT the LCA, check and go down the tree
        if p < node.val && q < node.val {
            curr_node = node.left;
        } else {
            curr_node = node.right;
        }
    }
    0
}
s