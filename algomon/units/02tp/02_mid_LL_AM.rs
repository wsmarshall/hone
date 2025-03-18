use std::error;
use std::io;
use std::str::FromStr;

type List<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,

    next: List<T>,
}

fn middle_of_linked_list(head: List<i32>) -> i32 {
    let mut slow = &head;

    let mut fast = &head;

    while let Some(curr) = fast {
        if let Some(next) = &curr.next {
            fast = &next.next;

            slow = &slow.as_ref().unwrap().next;
        } else {
            break;
        }
    }

    slow.as_ref().unwrap().val
}
