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

    //iterative method for building an n-ary tree
    fn build_tree(&mut self, input: String, n_ary: usize) {
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
                    if (num_leaves >= n_ary) {
                        //leafs are full on current node
                        //current parent is now prev
                        current_parent = parent_indices.pop();
                    } else {
                        //current is a leaf of prev node placed
                        current_parent = parent_indices[parent_indices.len() - 1];
                    }
                } else {
                    //setting the root node up, starting parent index stack
                    parent_indices.push(current_index);
                }
                //add current node to tree
                self.arena.push(Node::new(current_index, list[i]));
                //mark current node in parent
                self.arena[current_parent].children.push(current_index);

                //update place-marking index
                current_index += 1;
            }
        }
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
}
