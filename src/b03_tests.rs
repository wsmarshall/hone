#[cfg(test)]

mod bo3_tests {

    use crate::b03::first_not_smaller;

    #[test]
    fn one() {
        let list: [usize; 7] = [1, 3, 3, 5, 8, 8, 10];
        let target = 2;
        //should return 2
        assert_eq!(first_not_smaller(&list, target), 1);
    }

    #[test]
    fn two() {
        let list: [usize; 1] = [0];
        let target = 0;
        //should return 0
        assert_eq!(first_not_smaller(&list, target), 0);
    }

    #[test]
    fn three() {
        let list: [usize; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let target = 10;
        //should return 9
        assert_eq!(first_not_smaller(&list, target), 9);
    }

    #[test]
    fn four() {
        let list: [usize; 12] = [1, 2, 2, 2, 2, 2, 2, 3, 5, 8, 8, 10];
        let target = 2;
        //should return 1
        assert_eq!(first_not_smaller(&list, target), 1);
    }
}
