/**
 * Find the middle node of a linked list.

Input: 0 1 2 3 4

Output: 2

If the number of nodes is even, then return the second middle node.

Input: 0 1 2 3 4 5

Output: 3
 */
use std::error;
use std::io;
use std::str::FromStr;

type List<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: List<T>,
}

fn middle_of_linked_list(head: List<i32>) -> i32 {
    let mut fast = &head;
    let mut slow = &head;
    while let Some(n) = *fast.next {
        slow = n.next;
        fast = n.next;
        if let Some(o) = *fast.next {
            fast = o.next;
        }
    }
    slow.val
}
