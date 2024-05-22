/**
 * a node for trees
 */

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    index: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    pub fn new(val: Option<i32>, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node {
        Node {
            val: val,
            left: left,
            right: right,
        }
    }

    fn value(&self) -> i32 {
        self.val.unwrap()
    }

    fn left(&self) -> Option<Box<Node>> {
        if self.left == None {
            return None;
        }
        return self.left;
    }

    fn right(&self) -> Option<Box<Node>> {
        if self.right == None {
            return None;
        }
        return self.right;
    }

    fn in_order_traversal(&self, root: Option<Box<Node>>) {
        if !(root == None) {
            self.in_order_traversal(root.clone().unwrap().left());
            println!("{}", root.unwrap().value());
            self.in_order_traversal(root.clone().unwrap().right());
        }
    }
}
