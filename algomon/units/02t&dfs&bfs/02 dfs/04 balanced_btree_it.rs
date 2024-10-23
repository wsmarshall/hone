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
        let mut dstack = Vec::new(); //depth stack for right depths, holds i32s
        let mut stack = vec![(root, 0 as i32, 0 as i32, false, false)];
        while let Some((node, depth, left_depth, seen_left, seen_right)) = stack.pop() {
            if let Some(nval) = node.clone() {
                if !seen_left {
                    stack.push((node.clone(), depth, left_depth, true, false));
                    stack.push((
                        nval.borrow().left.clone(),
                        depth + 1,
                        left_depth,
                        false,
                        false,
                    ));
                } else if !seen_right {
                    stack.push((node.clone(), depth, dstack.pop().unwrap(), true, true));
                    stack.push((
                        nval.borrow().right.clone(),
                        depth + 1,
                        left_depth,
                        false,
                        false,
                    ));
                } else {
                    let ldepth = left_depth;
                    let rdepth = dstack.pop().unwrap();
                    if (ldepth - rdepth).abs() > 1 {
                        return false;
                    }
                    dstack.push(std::cmp::max(ldepth, rdepth));
                }
            } else {
                dstack.push(depth - 1);
            }
        }
        true
    }
}
