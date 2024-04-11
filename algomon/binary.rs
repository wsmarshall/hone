fn vanilla_binary(list: &vec) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_present(f) -> Result<(), i32> {
        let mut list: vec![1, 3, 5, 7, 8];
        let target = 5;
        
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(-1)
        }
    }

    fn result_missing() -> Result<(), i32> {
        let mut list: vec![1, 2, 3, 4,5, 6, 7];
        let target = 0;
    }
}