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
        let mut stack = Vec::new();
        stack.push(root);

        while let Some(Some(node)) = stack.pop() {
            let b = node.borrow();

            if b.val == val {
                return Some(Rc::clone(&node));
            }

            if let Some(left) = &b.left {
                stack.push(Some(Rc::clone(&left)));
            }

            if let Some(right) = &b.right {
                stack.push(Some(Rc::clone(&right)));
            }
        }
        None
    }
}
