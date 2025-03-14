def build_tree_recursive(preorder_index: int, inorder_start: int, size: int, value_to_index: dict) -> Optional[Node]:

    if size <= 0:

        return None


    root_value = preorder[preorder_index]

    inorder_root_index = value_to_index[root_value]

    left_subtree_size = inorder_root_index - inorder_start


    left_child = build_tree_recursive(preorder_index + 1, inorder_start, left_subtree_size, value_to_index)

    right_child = build_tree_recursive(preorder_index + 1 + left_subtree_size, inorder_root_index + 1, size - 1 - left_subtree_size, value_to_index)


    return Node(root_value, left_child, right_child)


def construct_binary_tree(preorder: List[int], inorder: List[int]) -> Optional[Node]:

    value_to_index = {val: idx for idx, val in enumerate(inorder)}


    return build_tree_recursive(0, 0, len(preorder), value_to_index)