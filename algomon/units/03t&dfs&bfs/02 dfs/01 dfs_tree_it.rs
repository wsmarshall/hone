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

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![root];

        while let Some(Some(rc)) = stack.pop() {
            let node = rc.borrow();

            if node.val == val {
                return Some(rc.clone());
            }

            if node.right.is_some() {
                stack.push(node.right.clone());
            }

            if node.left.is_some() {
                stack.push(node.left.clone());
            }
        }

        None
    }
}
