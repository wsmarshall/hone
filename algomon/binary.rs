fn vanilla_binary(list: &vec, i: i32) -> i32 {
    //returns index of target
    //or -1 if not found
    0 //test filler value
}

#[cfg(test)]
mod tests {

    #[test]
    fn result_present() {
        let mut list: vec![1, 3, 5, 7, 8];
        let target = 5;

        //should return 2
        assert_eq!(vanilla_binary(list, target), 2);
    }

    #[test]
    fn result_missing() {
        let mut list: vec![1, 2, 3, 4, 5, 6, 7];
        let target = 0;
        //should return -1
        assert_eq!(vanilla_binary(list, target), 1);
    }

    #[test]
    fn larger_array() {
        let mut list: vec![2, 8, 89, 120, 1000];
        let target = 120;
        //should return 3
    }

    #[test]
    fn smaller_array() {
        let mut list: vec![10, 20];
        let target = 20;
        //should return 1
    }

    #[test]
    fn smallest_array() {
        let mut list: vec![1];
        let target = 1;
        //should return 0
    }
}
