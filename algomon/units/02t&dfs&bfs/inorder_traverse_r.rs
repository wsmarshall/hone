// Definition for a binary tree node.
// PROVIDED
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

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
