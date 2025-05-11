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
    // WRITE YOUR BRILLIANT CODE HERE
    0
}
