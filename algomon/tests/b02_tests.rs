#[cfg(test)]

mod bo2_tests {

    use crate::b02::find_boundary;

    #[test]
    fn result_present() {
        let list: [bool; 5] = [false, false, true, true, true];

        //should return 2
        assert_eq!(find_boundary(&list), Some(2));
    }

    #[test]
    fn single_entry_true() {
        let list: [bool; 1] = [true];

        //should return 0
        assert_eq!(find_boundary(&list), Some(0));
    }

    #[test]
    fn only_false() {
        let list: [bool; 3] = [false, false, false];

        //should return None
        assert_eq!(find_boundary(&list), None);
    }

    #[test]
    fn only_true() {
        let list: [bool; 5] = [true, true, true, true, true];

        //should return 0
        assert_eq!(find_boundary(&list), Some(0));
    }

    #[test]
    fn false_true() {
        let list: [bool; 2] = [false, true];

        //should return 1
        assert_eq!(find_boundary(&list), Some(1));
    }

    #[test]
    fn last_true() {
        let list: [bool; 9] = [false, false, false, false, false, false, false, false, true];
        //should return 8
        assert_eq!(find_boundary(&list), Some(8));
    }
}
