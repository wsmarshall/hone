// Definition for a binary tree node.
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

//LC 145

use std::cell::RefCell;
use std::rc::Rc;

type: OptioNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        Self::preorder_helper(&root, &mut v);
        v
    }

    fn preorder_helper(node: &OptioNode, v: &mut Vec<i32>) {
        if let Some(n) = node {
            let b = n.borrow();
            v.push(b.val);
            Self::preorder_helper(&b.left, v);
            Self::preorder_helper(&b.right, v);
        }
    }
}
