mod node;

fn main() {

    let mut test_tree = ArenaTree {
        arena: Vec::new();
    }
    test_tree.build_tree("5 4 3 x x 8 x x 6 x x", 2);

    println!("test tree is: {}", test_tree.to_string);
}
