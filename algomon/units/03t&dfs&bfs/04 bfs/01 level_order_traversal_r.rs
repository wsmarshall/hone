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
    let mut answer: Vec<Vec<i32>> = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let mut level_list = vec![];
        for i in 0..queue.len() {
            if let Some(Some(current_node)) = queue.pop_front() {
                level_list.push(current_node.val);

                if current_node.left.is_some() {
                    queue.push_back(current_node.left);
                }

                if current_node.right.is_some() {
                    queue.push_back(current_node.right);
                }
            }
        }
        answer.push(level_list);
    }
    answer
}
