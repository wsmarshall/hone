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
    // WRITE YOUR BRILLIANT CODE HERE
    Vec::new()
}
