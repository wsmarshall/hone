use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = HashSet::new();
        let mut cols = HashSet::new();
        let mut sub_box = vec![HashSet::new(); 9];

        for r in (0..9) {
            for c in (0..9) {
                if let Some(digit) = board[r][c].to_digit(10) {
                    if !rows.insert((r, digit)) {
                        return false;
                    }
                    if !cols.insert((c, digit)) {
                        return false;
                    }
                    if !sub_box[3 * (r / 3) + (c / 3)].insert(digit) {
                        return false;
                    }
                }
            }
        }
        true
    }
}
