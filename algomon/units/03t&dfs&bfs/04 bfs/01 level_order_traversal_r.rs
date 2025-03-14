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
    // with push_back and pop_front for queue implementation
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut level_list: Vec<Vec<i32>>::new();
    
    while !queue.is_empty() {
        let n = queue.len();
        let mut next_level = Vec<i32>::new();
        for i in 0..n {
            let new_node = queue.pop_front();
            next_level.push(new_node.val);
            if new_node.left.is_some() {
                queue.push_back(new_node.left);
            }
            if new_node.right.is_some() {
                queue.push_back(new_node.right);
            }
        }
        level_list.push(next_level);
    }


    level_list



}