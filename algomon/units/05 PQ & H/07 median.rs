//this is LC since AM has no Rust impl testing

use std::collections::BinaryHeap;
use std::cmp::Reverse;


struct MedianFinder {
    big_pile: BinaryHeap<Reverse<i32>>,
    small_pile: BinaryHeap<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        MedianFinder{
            big_pile: BinaryHeap::new(),
            small_pile: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {  
        if let Some(biggest_small) = self.small_pile.peek() {
            if num > *biggest_small {
                self.big_pile.push(Reverse(num));
                // println!("big pile: {:?}", self.big_pile);

            } else {
                self.small_pile.push(num);
            }
        } else {
            self.small_pile.push(num);
        } 

        let big_size = self.big_pile.len();
        let small_size = self.small_pile.len();

        if small_size > big_size + 1 {
            let temp = self.small_pile.pop().unwrap();
            self.big_pile.push(Reverse(temp));
        } else if big_size > small_size {
            let Reverse(temp) = self.big_pile.pop().unwrap();
            self.small_pile.push(temp);
        }
        // println!("small pile, big pile: {:?}, {:?}", self.small_pile, self.big_pile);
    }
    
    fn find_median(&mut self) -> f64 {
        let big_size = self.big_pile.len();
        let small_size = self.small_pile.len();

        if small_size == big_size + 1 {
            return *(self.small_pile.peek().unwrap()) as f64;
        } else if small_size == big_size {
            let first: f64 = *(self.small_pile.peek().unwrap()) as f64;
            let Reverse(second) = self.big_pile.peek().unwrap(); 
            let second: f64 = *second as f64;
            return (first + second)/2.0;

        } else {//for compiler, don't expect to need this
            return 0.0
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */