mod node;

fn main() {
    use crate::node::ArenaTree;

    let mut test_tree = ArenaTree { arena: Vec::new() };

    test_tree.build_tree("1 2 4 x x 5 x x 3 6 x x 7 x x", 2);

    // println!(
    //     "in order traversal of test tree is: {}",
    //     test_tree.in_order_traversal_iterative()
    // );
}
