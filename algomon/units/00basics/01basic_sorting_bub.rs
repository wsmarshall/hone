//quick vanilla bubble sort

fn sort_list(unsorted_list: Vec<i32>) -> Vec<i32> {
    let mut list = unsorted_list.clone();
    let len = unsorted_list.len();

    for i in 0..len {
        for j in 0..(len - 1 - i) {
            if (list[j] > list[j + 1]) {
                let temp = list[j];
                list[j] = list[j + 1];
                list[j + 1] = temp;
            }
        }
    }

    list
}
