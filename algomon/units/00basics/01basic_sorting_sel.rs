//quick vanilla selection sort

fn sort_list(unsorted_list: Vec<i32>) -> Vec<i32> {
    let mut list = unsorted_list.clone();

    let mut mindex;
    for i in 0..list.len() {
        mindex = i;

        for j in i..list.len() {
            if list[j] < list[mindex] {
                mindex = j;
            }
        }

        if mindex != i {
            let temp = list[i];
            list[i] = list[mindex];
            list[mindex] = temp;
        }
    }

    list
}
