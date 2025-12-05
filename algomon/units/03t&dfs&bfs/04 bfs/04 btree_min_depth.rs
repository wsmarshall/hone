use std::collections::VecDeque;
use std::error;
use std::io;
use std::str::FromStr;

type Tree<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    left: Tree<T>,
    right: Tree<T>,
}

fn binary_tree_min_depth(root: Tree<i32>) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut depth = 0;
    while !queue.is_empty() {
        for i in 0..queue.len() {
            if let Some(Some(n)) = queue.pop_front() {
                if n.left.is_none() && n.right.is_none() {
                    return depth;
                }

                if n.left.is_some() {
                    queue.push_back(n.left);
                }
                if n.right.is_some() {
                    queue.push_back(n.right);
                }
            }
        }
        depth += 1;
    }
    -1
}
