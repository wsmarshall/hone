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
}
