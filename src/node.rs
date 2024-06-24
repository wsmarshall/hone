/**
 * a memory arena based approach for nodes/graphs in Rust
 * used for tree construction and graph algos
 */

#[derive(Debug, Default)]
pub struct ArenaTree {
    pub arena: Vec<Node>,
}

impl ArenaTree {
    pub fn node(&mut self, val: u32) -> usize {
        //check if given node exists
        for node in &self.arena {
            if node.val == val {
                return node.index;
            }
        }

        //add it if it doesn't exist
        let index = self.arena.len();
        self.arena.push(Node::new(index, val));
        index
    }

    //iterative method for building an n-ary tree
    pub fn build_tree(&mut self, input: &str, n_ary: usize) {
        if n_ary == 2 {
            //a binary tree
            const RADIX: u32 = 10;
            let list: Vec<&str> = input.split(' ').collect();
            //stack for parent node indices
            let mut parent_indices = Vec::new();
            let mut current_index = 0;
            let mut num_leaves = 0;
            let mut current_parent = 0;
            let mut num_children = 0;

            for i in 0..list.len() {
                //no leaf node marker
                if list[i] == "x" {
                    num_leaves += 1;
                    if num_leaves >= n_ary {
                        if parent_indices.len() > 1 {
                            parent_indices.pop();
                            current_parent = parent_indices[parent_indices.len() - 1];
                        } else {
                            //is at root for parent
                            current_parent = parent_indices[0];
                        }

                        num_leaves = 0;
                        continue;
                    }
                } else {
                    if i > 0 {
                        //not the root node in the tree
                        if num_leaves >= n_ary || num_children >= n_ary {
                            if num_leaves >= n_ary {
                                //leafs are full on current node
                                num_leaves = 0;
                            }

                            if num_children >= n_ary {
                                //parent node is "full"
                                num_children = 0;
                            }

                            //current parent is now prev
                            current_parent = parent_indices.pop().unwrap();
                        } else {
                            //current is a leaf of prev node placed
                            num_children += 1;
                            //add current node to tree
                            current_index =
                                self.node(list[i].chars().nth(0).unwrap().to_digit(RADIX).unwrap());
                            //mark parent
                            self.arena[current_index].parent = Some(current_parent);
                            //mark current node in parent
                            self.arena[current_parent].children.push(current_index);

                            parent_indices.push(current_index);

                            current_parent = current_index;
                            num_leaves = 0;

                            continue;
                        }
                    } else {
                        //setting the root node up, starting parent index stack
                        parent_indices.push(current_index);
                        //add current node to tree
                        current_index =
                            self.node(list[i].chars().nth(0).unwrap().to_digit(RADIX).unwrap());
                        continue;
                    }
                    //add current node to tree
                    current_index =
                        self.node(list[i].chars().nth(0).unwrap().to_digit(RADIX).unwrap());
                    //mark parent
                    self.arena[current_index].parent = Some(current_parent);
                    //mark current node in parent
                    self.arena[current_parent].children.push(current_index);
                }
            }
        } else if (n_ary < 2) {
            //should never trigger, here for completeness
        } else {
            //an n-ary tree for n > 2
            //the actual numbers
            let list: Vec<&str> = input.split(' ').collect();

            //"triple" tuple structs for nodes, children, and counts
            struct Triple(u32, u32, u32);

            //keeps track of "triple" tuple structs for nodes and children
            //where .0 is node value, .1 is num children, and .2 is current count
            let mut tracker = Vec::new();
            let mut curr_parent = 0;
            //acting as a stack
            let mut curr_parent_stack = Vec::new();
            let mut num_children = 0;
            let mut i = 0; //for accessing/iterating 'list'
            let mut j = 0; //for keeping track of children count
            while i < list.len() {
                //insert current node into tree, record index and val
                let curr_index = self.node(list[i].chars().nth(0).unwrap().to_digit(10).unwrap());
                let curr_val = self.arena[curr_index].val;

                if list[i + 1].chars().nth(0).unwrap().to_digit(10).unwrap() > 0 {
                    //has children
                    num_children = list[i + 1].chars().nth(0).unwrap().to_digit(10).unwrap();
                    curr_parent = curr_index;
                    curr_parent_stack.push(curr_parent);
                    let mut current = Triple(curr_val, num_children, j);
                    tracker.push(current);
                    //reset j
                    j = 0;
                } else {
                    //no children
                    //mark parent
                    self.arena[curr_parent].children.push(curr_index);
                    j += 1;
                }
                i += 2;
            }
        }

        for i in &self.arena {
            println!(
                "node at {}, value: {}, parent: {:?}, children: {:?}",
                i.index, i.val, i.parent, i.children
            );
        }
        println!();
        println!();
        println!();
    }

    //for printing out a string representation of a tree
    pub fn to_string(&self) -> String {
        let mut tree = String::from("");

        for i in &self.arena {
            tree += &i.val.to_string();
            tree += " \n";
        }
        tree
    }

    //gives back how many nodes in the tree
    fn size(&self) -> usize {
        self.arena.len()
    }

    //this version assumes binary tree
    pub fn in_order_traversal_iterative(&self) -> String {
        if self.arena.len() == 0 {
            return "".to_string();
        }
        let num_nodes = self.size();
        let mut count = 0;

        let mut traverse = String::from("\n");

        //for behaving like a stack
        let mut tracker = Vec::new();

        let mut current = self.arena[0].index;

        while count < num_nodes {
            if self.arena[current].children.len() > 0 {
                //current node has a left child
                tracker.push(current);
                //set current to current's left child
                current = self.arena[current].children[0];
            } else {
                //current node has no left child

                traverse += &self.arena[current].val.to_string();
                traverse += "\n";
                count += 1;

                if !tracker.is_empty() {
                    //current becomes parent of node with no left child
                    current = tracker.pop().unwrap();
                    traverse += &self.arena[current].val.to_string();
                    traverse += "\n";
                    count += 1;
                }
                //check for right child
                if self.arena[current].children.len() > 1 {
                    //if right child, current becomes
                    current = self.arena[current].children[1];
                    // tracker.push(self.arena[current].children[2]);
                } else {
                    //no right children - current becomes parent
                    if !tracker.is_empty() {
                        current = tracker.pop().unwrap();
                    }
                }
            }
        }

        traverse
    }

    //this version assumes binary tree
    pub fn pre_order_traversal_iterative(&self) -> String {
        if self.arena.len() == 0 {
            return "".to_string();
        }

        //also for pinging with 'contains'
        let mut traverse = String::from("\n");

        //for behaving like a stack
        let mut tracker = Vec::new();
        //dummy placeholder variable
        tracker.push(0);

        let mut current = self.arena[0].index;

        //add self
        traverse += &self.arena[current].val.to_string();
        traverse += "\n";
        tracker.push(current);

        while !tracker.is_empty() {
            if self.arena[current].children.len() > 0
                && !traverse.contains(
                    &self.arena[*&self.arena[current].children[0]]
                        .val
                        .to_string(),
                )
            {
                //set current to current's left child
                current = self.arena[current].children[0];
            } else {
                //current node has no left child

                //check for right child
                if self.arena[current].children.len() > 1
                    && !traverse.contains(
                        &self.arena[*&self.arena[current].children[1]]
                            .val
                            .to_string(),
                    )
                {
                    //if right child, current becomes
                    current = self.arena[current].children[1];
                } else {
                    //no right children - current becomes parent
                    if !tracker.is_empty() {
                        current = tracker.pop().unwrap();
                    }
                }
            }
            if !traverse.contains(&self.arena[current].val.to_string()) {
                //add self
                traverse += &self.arena[current].val.to_string();
                traverse += "\n";
                tracker.push(current);
            }
        }
        traverse
    }

    //this version assumes binary tree
    pub fn post_order_traversal_iterative(&self) -> String {
        if self.arena.len() == 0 {
            return "".to_string();
        }

        //also for pinging with 'contains'
        let mut traverse = String::from("\n");

        //for behaving like a stack
        let mut tracker = Vec::new();

        //set current equal to the root
        let mut current = self.arena[0].index;

        let mut has_left = false;
        let mut has_right = false;

        while traverse.len() < self.size() || !tracker.is_empty() {
            if self.arena[current].children.len() > 1
                && !traverse.contains(&self.arena[self.arena[current].children[1]].val.to_string())
            {
                //current has at least a right child
                if !traverse.contains(&self.arena[current].val.to_string()) {
                    has_right = true;
                    if !tracker.contains(&current) {
                        tracker.push(current);
                    }
                    if !tracker.contains(&self.arena[current].children[1]) {
                        tracker.push(self.arena[current].children[1]);
                    }
                }
            }

            if self.arena[current].children.len() > 0
                && !traverse.contains(&self.arena[self.arena[current].children[0]].val.to_string())
            {
                //current has at least a left child
                if !traverse.contains(&self.arena[self.arena[current].children[0]].val.to_string())
                {
                    has_left = true;
                    if !tracker.contains(&current) {
                        tracker.push(current);
                    }
                }
            }

            if has_left {
                //set current to left child
                current = self.arena[current].children[0];
            } else if !has_left && has_right {
                //set current to right child
                current = self.arena[current].children[1]
            } else {
                if !traverse.contains(&self.arena[current].val.to_string()) {
                    traverse += &self.arena[current].val.to_string();
                    traverse += "\n";
                }
                current = tracker.pop().unwrap();
            }
            if tracker.is_empty() {
                traverse += &self.arena[current].val.to_string();
            } else {
                has_left = false;
                has_right = false;
            }
        }
        traverse
    }
}

#[derive(Debug)]
pub struct Node {
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
