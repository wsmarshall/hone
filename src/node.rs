/**
 * an arena based approach for nodes/graphs in Rust
 */

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn node(&mut self, val: T) -> usize {
        //check if it exists
        for node in &self.arena {
            if node.val == val {
                return node.index;
            }
        }

        //add it if !exist
        let index = self.arena.len();
        self.arena.push(Node::new(index, val));
        index
    }
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    index: usize,
    val: T,
    parent: Option<usize>,
    //left child, then right child in that order in the Vec
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn new(index: usize, val: T) -> Self {
        Self {
            index,
            val,
            parent: None,
            children: vec![],
        }
    }

    // fn value(&self) -> i32 {
    //     self.val.unwrap()
    // }

    // fn left(&self) -> Option<Box<Node>> {
    //     if self.left == None {
    //         return None;
    //     }
    //     return self.left;
    // }

    // fn right(&self) -> Option<Box<Node>> {
    //     if self.right == None {
    //         return None;
    //     }
    //     return self.right;
    // }

    // fn in_order_traversal(&self, root: Option<Box<Node>>) {
    //     if !(root == None) {
    //         self.in_order_traversal(root.clone().unwrap().left());
    //         println!("{}", root.unwrap().value());
    //         self.in_order_traversal(root.clone().unwrap().right());
    //     }
    // }
}
