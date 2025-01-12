//a quick insertion sort

fn sort_list(unsorted_list: Vec<i32>) -> Vec<i32> {
    //cloning is completely optional; can use the raw list
    let mut list = unsorted_list.clone();

    let mut i = 1;
    while i < list.len() {
        let mut j = i;
        while j > 0 && list[j] < list[j - 1] {
            let temp = list[j - 1];
            list[j - 1] = list[j];
            list[j] = temp;
            j -= 1;
        }
        i += 1;
    }

    list
}
