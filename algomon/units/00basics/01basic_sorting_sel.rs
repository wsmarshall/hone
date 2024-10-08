//quick vanilla selection sort

fn sort_list(unsorted_list: Vec<i32>) -> Vec<i32> {
    let mut list = unsorted_list.clone();

    for i in 0..list.len() {
        let mut min: i32 = list[i];
        let mut mindex = i;

        for j in i..list.len() {
            if list[j] < min {
                min = list[j];
                mindex = j;
            }
        }

        if min < list[i] {
            let temp = list[i];
            list[i] = min;
            list[mindex] = temp;
        }
    }

    list
}
