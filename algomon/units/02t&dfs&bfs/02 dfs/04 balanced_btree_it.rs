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
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut dstack = Vec::new();
        let mut stack = Vec::new();
        stack.push((1 as i32, 0 as i32, false, false, root));
        while let Some((depth, left_depth, seen_left, seen_right, node)) = stack.pop() {
            if let Some(nval) = node.clone() {
                if !seen_left {
                    stack.push((depth, left_depth, true, false, node.clone()));
                    stack.push((
                        depth + 1,
                        left_depth,
                        false,
                        false,
                        nval.borrow().left.clone(),
                    ));
                } else if !seen_right {
                    stack.push((depth, dstack.pop().unwrap(), true, true, node.clone()));
                    stack.push((
                        depth + 1,
                        left_depth,
                        false,
                        false,
                        nval.borrow().right.clone(),
                    ));
                } else {
                    let ldepth = left_depth;
                    let rdepth = dstack.pop().unwrap();
                    if 1 < (ldepth - rdepth).abs() {
                        return false;
                    }
                    dstack.push(ldepth.max(rdepth));
                }
            } else {
                dstack.push(depth - 1);
            }
        }
        return true;
    }
}
