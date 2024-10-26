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
//LC 104 or AM eponymous

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        let mut stack = vec![(root, 0)];
        while let Some((node, mut depth)) = stack.pop() {
            if let Some(n) = node {
                let b = n.borrow();
                depth += 1;
                max_depth = std::cmp::max(max_depth, depth);

                stack.push((b.right.clone(), depth));
                stack.push((b.left.clone(), depth));
            }
        }
        max_depth
    }
}
