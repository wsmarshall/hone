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
use std::cell::RefCell;
use std::rc::Rc;

type OptioNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        Self::postorder_helper(&root, &mut v);
        v
    }

    fn postorder_helper(node: &OptioNode, v: &mut Vec<i32>) {
        if let Some(n) = node {
            let b = n.borrow();
            Self::postorder_helper(&b.left, v);
            Self::postorder_helper(&b.right, v);
            v.push(b.val);
        }
    }
}
