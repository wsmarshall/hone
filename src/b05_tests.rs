#[cfg(test)]

mod bo5_tests {

    use crate::b05::square_root

    #[test]
    fn one() {
        let target = 4;
        //should return 2
        assert_eq!(first_occurrence(target), Some(2));
    }

    #[test]
    fn two() {
        let target = 8;
        //should return 2
        assert_eq!(first_occurrence(target), Some(2));
    }

    #[test]
    fn three() {
        let target = 10;
        //should return 3
        assert_eq!(first_occurrence(target), Some(3));
    }

    #[test]
    fn four() {
        let target = 0;
        //should return 0
        assert_eq!(first_occurrence(target), Some(0));
    }

    #[test]
    fn five() {
        let target = 1;
        //should return 1
        assert_eq!(first_occurrence(target), Some(1));
    }

    // #[test]
    // fn six() {
    //     println!("test 6");
    //     let list: [usize; 1] = [4];
    //     let target = 4;
    //     //should return 0
    //     assert_eq!(first_occurrence(&list, target), Some(0));
    // }

    // #[test]
    // fn seven() {
    //     println!("test 7");
    //     let list: [usize; 5] = [2, 3, 5, 7, 11];
    //     let target = 2;
    //     //should return 0
    //     assert_eq!(first_occurrence(&list, target), Some(0));
    // }

    // #[test]
    // fn eight() {
    //     println!("test 8");
    //     let list: [usize; 6] = [1, 3, 5, 8, 13, 21];
    //     let target = 40;
    //     //should return None
    //     assert_eq!(first_occurrence(&list, target), None);
    // }
}
