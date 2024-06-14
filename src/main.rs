mod node;

fn main() {
    use crate::node::ArenaTree;

    // let mut test_tree1 = ArenaTree { arena: Vec::new() };

    // test_tree1.build_tree("5 4 3 x x 8 x x 6 x x", 2);

    // println!(
    //     "post-order traversal of test tree is: {}",
    //     test_tree1.post_order_traversal_iterative()
    // );

    let mut test_tree2 = ArenaTree { arena: Vec::new() };

    test_tree2.build_tree("1 2 4 x x 5 x x 3 6 x x 7 x x", 2);

    println!(
        "post-order traversal of test tree is: {}",
        test_tree2.post_order_traversal_iterative()
    );
}
