use std::cell::RefCell;
use std::rc::Rc;
type OptNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn inorder_traversal(root: OptNode) -> Vec<i32> {
        let mut v = Vec::new();
        let mut stack = vec![(root, false)];
        while let Some((node, push_now)) = stack.pop() {
            if push_now {
                v.push(node.unwrap().borrow().val);
            } else if let Some(n) = node {
                let b = n.borrow();
                stack.push((b.right.clone(), false));
                stack.push((Some(n.clone()), true));
                stack.push((b.left.clone(), false));
            }
        }
        v
    }
}
