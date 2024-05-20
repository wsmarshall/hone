// // #[cfg(test)]

// mod binary01_tests {

//     use crate::binary01::vanilla_binary;

//     #[test]
//     fn result_present() {
//         let list: [usize; 5] = [1, 3, 5, 7, 8];
//         let target: usize = 5;

//         //should return 2
//         assert_eq!(vanilla_binary(&list, target).unwrap_or(11), 2);
//     }

//     #[test]
//     fn result_missing() {
//         let list: [usize; 7] = [1, 2, 3, 4, 5, 6, 7];
//         let target: usize = 0;

//         //should return None
//         assert_eq!(vanilla_binary(&list, target), None);
//     }

//     #[test]
//     fn larger_array() {
//         let list: [usize; 5] = [2, 8, 89, 120, 1000];
//         let target: usize = 120;

//         //should return 3
//         assert_eq!(vanilla_binary(&list, target).unwrap_or(11), 3);
//     }

//     #[test]
//     fn smaller_array() {
//         let list: [usize; 2] = [10, 20];
//         let target: usize = 20;

//         //should return 1
//         assert_eq!(vanilla_binary(&list, target).unwrap_or(11), 1);
//     }

//     #[test]
//     fn smallest_array() {
//         let list: [usize; 1] = [1];
//         let target: usize = 1;

//         //should return 0
//         assert_eq!(vanilla_binary(&list, target).unwrap_or(11), 0);
//     }

//     #[test]
//     fn empty_array() {
//         let list = [];
//         let target = 1;

//         //should return None
//         assert_eq!(vanilla_binary(&list, target), None);
//     }

//     #[test]
//     fn not_found() {
//         let list: [usize; 5] = [1, 2, 3, 4, 5];
//         let target: usize = 10;

//         //should return None
//         assert_eq!(vanilla_binary(&list, target), None);
//     }
// }
