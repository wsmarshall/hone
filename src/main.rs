mod b04;
mod b04_tests;

fn main() {
    use crate::b04::first_occurrence;

    let list: [usize; 7] = [1, 22, 22, 33, 50, 100, 20000];
    let target = 33;
    //should return 3
    assert_eq!(first_occurrence(&list, target), Some(3));
}
