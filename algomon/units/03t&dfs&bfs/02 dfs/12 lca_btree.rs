//LC 236
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let pval = p.clone().unwrap().borrow().val;
        let qval = q.clone().unwrap().borrow().val;

        if let Some(rc) = root {
            let node = rc.borrow();
            let nval = node.val;

            if nval == pval || nval == qval {
                return Some(rc.clone());
            }

            let left = Self::lowest_common_ancestor(node.left.clone(), p.clone(), q.clone());
            let right = Self::lowest_common_ancestor(node.right.clone(), p.clone(), q.clone());

            if left.is_some() && right.is_some() {
                return Some(rc.clone());
            } else if left.is_some() {
                return left;
            } else if right.is_some() {
                return right;
            }
        }

        None
    }
}
