#[cfg(test)]

mod bo7_tests {

    use crate::b07::find_peak;

    #[test]
    fn one() {
        let list = [0, 1, 2, 3, 2, 1, 0];
        //should return 3
        assert_eq!(find_peak(&list), 3);
    }

    #[test]
    fn two() {
        let list = [0, 10, 3, 2, 1, 0];
        //should return 0
        assert_eq!(find_peak(&list), 1);
    }

    #[test]
    fn three() {
        let list = [0, 10, 0];
        //should return 0
        assert_eq!(find_peak(&list), 1);
    }

    #[test]
    fn four() {
        let list = [
            0, 1, 2, 12, 22, 32, 42, 52, 62, 72, 82, 92, 102, 112, 122, 132, 133, 132, 111, 0,
        ];
        //should return 5
        assert_eq!(find_peak(&list), 16);
    }

    // #[test]
    // fn five() {
    //     let target = 1;
    //     //should return 1
    //     assert_eq!(find_min_rotated(target), 1);
    // }

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
