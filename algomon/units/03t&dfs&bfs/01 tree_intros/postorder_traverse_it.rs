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

//LC 144

use std::cell::RefCell;
use std::rc::Rc;

type OptioNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        let mut stack = vec![(root, false)];
        while let Some((node, push)) = stack.pop() {
            if push {
                v.push(node.unwrap().borrow().val);
            } else if let Some(n) = node {
                let b = n.borrow();
                //self
                stack.push((Some(n.clone()), true));
                //right
                stack.push((b.right.clone(), false));
                //left
                stack.push((b.left.clone(), false));
            }
        }
        v
    }
}
