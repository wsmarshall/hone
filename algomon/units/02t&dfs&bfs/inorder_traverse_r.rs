use std::cell::RefCell;
use std::rc::Rc;

//TreeNode is defined here?
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn inorder_traversal(root: OptNode) -> Vec<i32> {
        let mut v = Vec::new();
        Self::inorder(&root, &mut v);
        v
    }

    //helper function for recursive call
    fn inorder(node: &OptNode, v: &mut Vec<i32>) {
        if let Some(n) = node {
            //b is Option<Rc<Refcell<TreeNode>>>
            let b = n.borrow();
            Self::inorder(&b.left, v);
            v.push(b.val);
            Self::inorder(&b.right, v);
        }
    }
}
