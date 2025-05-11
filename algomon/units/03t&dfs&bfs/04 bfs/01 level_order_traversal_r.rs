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

fn level_order_traversal(root: Tree<i32>) -> Vec<Vec<i32>> {
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut traversal: Vec<Vec<i32>> = vec![];

    while !queue.is_empty() {
        let mut level: Vec<i32> = vec![];

        for i in 0..queue.len() {
            if let Some(Some(n)) = queue.pop_front() {
                level.push(n.val);

                if n.left.is_some() {
                    queue.push_back(n.left);
                }

                if n.right.is_some() {
                    queue.push_back(n.right);
                }
            }
        }
        traversal.push(level);
    }
    traversal
}
