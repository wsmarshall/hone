#[cfg(test)]

mod bo4_tests {

    use crate::b04::first_occurrence;

    #[test]
    fn one() {
        let list: [usize; 10] = [1, 3, 3, 3, 3, 6, 10, 10, 10, 100];
        let target = 3;
        //should return 1
        assert_eq!(first_occurrence(&list, target), Some(1));
    }

    #[test]
    fn two() {
        let list: [usize; 12] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let target = 1;
        //should return 0
        assert_eq!(first_occurrence(&list, target), Some(0));
    }

    #[test]
    fn three() {
        let list: [usize; 7] = [1, 22, 22, 33, 50, 100, 20000];
        let target = 33;
        //should return 3
        assert_eq!(first_occurrence(&list, target), Some(3));
    }

    #[test]
    fn four() {
        let list: [usize; 6] = [4, 6, 7, 7, 7, 20];
        let target = 8;
        //should return None
        assert_eq!(first_occurrence(&list, target), None);
    }

    #[test]
    fn five() {
        let list: [usize; 7] = [6, 7, 9, 10, 10, 10, 90];
        let target = 10;
        //should return 3
        assert_eq!(first_occurrence(&list, target), Some(3));
    }

    #[test]
    fn six() {
        let list: [usize; 1] = [4];
        let target = 4;
        //should return 0
        assert_eq!(first_occurrence(&list, target), Some(0));
    }

    #[test]
    fn seven() {
        let list: [usize; 5] = [2, 3, 5, 7, 11];
        let target = 2;
        //should return None
        assert_eq!(first_occurrence(&list, target), Some(0));
    }

    #[test]
    fn eight() {
        let list: [usize; 6] = [1, 3, 5, 8, 13, 21];
        let target = 40;
        //should return None
        assert_eq!(first_occurrence(&list, target), None);
    }
}
