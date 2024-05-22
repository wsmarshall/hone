/**
 * a node for trees
 */

struct Node {
    val: Option<i32>,
    left: Option<Node>,
    right: Option<Node>,
}

impl Node {
    pub fn new(val: Option<i32>, left: Option<i32>, right: Option<i32>) -> Node {
        Node {
            val: val,
            left: left,
            right: right,
        }
    }

    fn value(&self) -> i32 {
        unwrap(self.val)
    }

    fn left(&self) -> Option<Node> {
        if self.left == None {
            return None;
        }
        return self.left;
    }

    fn right(&self) -> Option<Node> {
        if self.right == None {
            return None;
        }
        return self.right;
    }

    fn in_order_traversal(&self, root: Option<Node>) {
        if !(root == None) {
            self.in_order_traversal(root.unwrap().left());
            println!("{}", root.unwrap().value());
            self.in_order_traversal(root.unwrap().right());
        }
    }
}
