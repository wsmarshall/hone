use std::collections::VecDeque;
use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

type Tree<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    left: Tree<T>,
    right: Tree<T>,
}

fn zig_zag_traversal(root: Tree<i32>) -> Vec<Vec<i32>> {
    let mut l2r: bool = true;
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut traversal: Vec<Vec<i32>> = vec![];

    while !queue.is_empty() {
        let mut next_level = vec![];
        for i in 0..queue.len() {
            if let Some(Some(n)) = queue.pop_front() {
                if l2r {
                    next_level.push(n.val);
                } else {
                    next_level.insert(0, n.val);
                }

                if n.left.is_some() {
                    queue.push_back(n.left);
                }

                if n.right.is_some() {
                    queue.push_back(n.right);
                }
            }
        }
        traversal.push(next_level);
        l2r = !l2r;
    }
    traversal
}
