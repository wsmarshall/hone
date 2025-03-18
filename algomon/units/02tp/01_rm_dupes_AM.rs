fn remove_duplicates(arr: &mut Vec<i32>) -> usize {
    if arr.is_empty() {
        return 0;
    }

    let mut write_index = 1;

    for i in 1..arr.len() {
        if arr[i] != arr[i - 1] {
            arr[write_index] = arr[i];

            write_index += 1;
        }
    }

    write_index
}
