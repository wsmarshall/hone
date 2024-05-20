mod b06;
mod b06_tests;

fn main() {
    use crate::b06::find_min_rotated;

    let list = [0, 1, 2, 3, 4, 5];
    //should return 0
    assert_eq!(find_min_rotated(&list), 0);
}
