//TODO counts cumulative left child heights, not heights of subtrees...

fn is_balanced(tree: Tree<i32>) -> bool {
    let mut left_h: i32 = 0;
    let mut right_h: i32 = 0;

    if let Some(node) = tree {
        //fields: node, height, left child, right child
        let mut stack = Vec::new();
        stack.push((node.right, false, true));
        stack.push((node.left, true, false));

        while let Some(entry) = stack.pop() {
            if let (n, is_left_child, is_right_child) = entry {
                if n.is_some() {
                    if is_left_child {
                        left_h += 1;
                        let b = n.unwrap();
                        stack.push((b.right, true, false));
                        stack.push((b.left, true, false));
                    } else {
                        right_h += 1;
                        let b = n.unwrap();
                        stack.push((b.right, false, true));
                        stack.push((b.left, false, true));
                    }
                }
            }
        }
    }

    //true if balanced
    (left_h - right_h).abs() <= 1
}
