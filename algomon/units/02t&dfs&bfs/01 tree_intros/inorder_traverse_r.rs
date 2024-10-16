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

type OptioNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn inorder_traversal(root: OptioNode) -> Vec<i32> {
        let mut v = Vec::new();
        Self::inorder_helper(&root, &mut v);
        v
    }

    fn inorder_helper(node: &OptioNode, v: &mut Vec<i32>) {
        if let Some(n) = node {
            //n is Rc<Refcell<TreeNode>>, b is Refcell<TreeNode>
            let b = n.borrow();
            Self::inorder_helper(&b.left, v);
            v.push(b.val);
            Self::inorder_helper(&b.right, v);
        }
    }
}
