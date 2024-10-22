//TODO non working example attemp modelled from The goog's AIOverview
/**
* def is_balanced(root):
   """
   Checks if a binary tree is balanced iteratively.

   A balanced binary tree is one in which the height of the left and right
   subtrees of every node differ by at most one.

   Args:
       root: The root node of the binary tree.

   Returns:
       bool: True if the tree is balanced, False otherwise.
   """

   if not root:
       return True

   stack = [(root, 0)]
   depth = {}

   while stack:
       node, level = stack.pop()

       if not node:
           continue

       if node not in depth:
           depth[node] = level

       if node.left:
           stack.append((node.left, level + 1))
       if node.right:
           stack.append((node.right, level + 1))

       if node.left in depth and node.right in depth:
           if abs(depth[node.left] - depth[node.right]) > 1:
               return False

   return True
*/
fn is_balanced(tree: Tree<i32>) -> bool {
    //node and level for tuple fields
    let mut stack = vec![(tree, 0)];
    let mut depth = HashMap::new();

    while let Some((Some(node), level)) = stack.pop() {
        if !depth.contains_key(&node.val) {
            depth.insert(node.val, level);
        }

        stack.push((node.left, level + 1));
        stack.push((node.right, level + 1));

        if depth.contains_key(&node.left.val) && depth.contains_key(&node.right.val) {
            if (depth.get(&node.left.val).unwrap() - depth.get(&node.right.val).unwrap()).abs > 1 {
                return false;
            }
        }
    }
    true
}
