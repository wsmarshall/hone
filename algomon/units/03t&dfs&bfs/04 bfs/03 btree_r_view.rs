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

fn binary_tree_right_side_view(root: Tree<i32>) -> Vec<i32> {
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut traversal: Vec<Vec<i32>> = vec![];
    let mut right_side_view: Vec<i32> = vec![];

    while !queue.is_empty() {
        let mut level = vec![];
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

    for mut i in traversal {
        right_side_view.push(i.pop().expect("bug"));
    }

    right_side_view
}
