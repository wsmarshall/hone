mod node;

fn main() {
    use crate::node::ArenaTree;

    let mut test_tree = ArenaTree { arena: Vec::new() };

    test_tree.build_tree("5 4 3 x x 8 x x 6 x x", 2);

    println!(
        "in order traversal of test tree is: {}",
        test_tree.in_order_traversal_iterative()
    );
}
