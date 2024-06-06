/**
 * a memory arena based approach for nodes/graphs in Rust
 */

#[derive(Debug, Default)]
struct ArenaTree {
    arena: Vec<Node>,
}

impl ArenaTree {
    pub fn node(&mut self, val: u32) -> usize {
        // commented out because we don't need
        // node values to be unique currently

        //  //check if it exists
        // for node in &self.arena {
        //     if node.val == val {
        //         return node.index;
        //     }
        // }

        //add it
        let index = self.arena.len();
        self.arena.push(Node::new(index, val));
        index
    }

    //iterative method for building an n-ary tree
    fn build_tree(&mut self, input: String, n_ary: usize) {
        const RADIX: u32 = 10;
        let list: Vec<&str> = input.split(' ').collect();
        //stack for parent node indices
        let mut parent_indices = Vec::new();
        let mut current_index = 0;
        let mut num_leaves = 0;
        let mut current_parent = 0;

        for i in 0..list.len() {
            //no leaf node marker
            if list[i] == "x" {
                num_leaves += 1;
                if num_leaves >= n_ary {
                    parent_indices.pop();
                    num_leaves = 0;
                    continue;
                }
            } else {
                if i > 0 {
                    //not the root node in the tree
                    if num_leaves >= n_ary {
                        //leafs are full on current node
                        //current parent is now prev
                        current_parent = parent_indices.pop().unwrap();
                    } else {
                        //current is a leaf of prev node placed
                        current_parent = parent_indices[parent_indices.len() - 1];
                    }
                } else {
                    //setting the root node up, starting parent index stack
                    parent_indices.push(current_index);
                }
                //add current node to tree
                current_index = self.node(list[i].chars().nth(0).unwrap().to_digit(RADIX).unwrap());
                //mark current node in parent
                self.arena[current_parent].children.push(current_index);
            }
        }
    }

    //for printing out a string representation of a tree
    pub fn to_string(&self) -> String {
        let mut tree = String::from("");

        for i in self.arena {
            tree += i.val;
            tree += " \n";
        }
        tree

    }

    fn level_traverse() -> Vec {
        //which level of tree, root = 0
        let mut level = 0;
        let mut nodes = Vec::new();
        let current
    }

    //gives back how many nodes in the tree
    fn size(&self) -> usize {
        self.arena.len()
    }


    //TODO re-implement below, iteratively
    // //this first pass version assumes binary tree
    // fn in_order_traversal<U>(&self, root: usize) {
    //     let mut traversal = Vec::new();
    //     self.in_order_traversal_helper::<U>(root, &mut traversal);

    //     let mut traverse = String::from("");
    //     for i in traversal {
    //         traverse.push(i as char::from_u32);
    //         traverse.push(' ');
    //     }
    //     println!("in-order traversal is: {}", traverse);
    // }

    // fn in_order_traversal_helper<U>(
    //     &self,
    //     n: usize,
    //     traversal: &mut Vec<usize>,
    // ) -> &mut Vec<usize> {
    //     self.in_order_traversal_helper::<U>(self.arena[n].children[0], traversal);
    //     traversal.push(n);
    //     self.in_order_traversal_helper::<U>(self.arena[n].children[1], traversal);

    //     traversal
    // }
}

#[derive(Debug)]
struct Node {
    index: usize,
    val: u32,
    parent: Option<usize>,
    //left child, then right child in that order in the Vec
    children: Vec<usize>,
}

impl Node {
    fn new(index: usize, val: u32) -> Self {
        Self {
            index,
            val,
            parent: None,
            children: vec![],
        }
    }
}
