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
//VIA LC 700
use std::cell::RefCell;
use std::rc::Rc;

type OptioNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn search_bst(root: OptioNode, val: i32) -> OptioNode {
        if root.is_none() {
            return None;
        }

        let binding = root.clone().unwrap();
        let b = binding.borrow();

        if b.val == val {
            return root;
        }

        let left = Self::search_bst(b.left.clone(), val);
        if left.is_some() {
            return left;
        }

        Self::search_bst(b.right.clone(), val)
    }
}
