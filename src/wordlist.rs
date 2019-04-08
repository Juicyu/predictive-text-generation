use std::collections::HashMap;
use core::borrow::BorrowMut;

pub struct WordList {
    words: HashMap<String, HashMap<String, usize>>,
}

impl WordList {
    //Public functions
    pub fn new() -> WordList {
        WordList {
            words: HashMap::new()
        }
    }

    pub fn add(&mut self, string_1: String, string_2: String) {
        if self.words.contains_key(&string_1) {
            let mut a= self.words.get_mut(&string_1).unwrap();

            if a.contains_key(&string_2) {
                a.insert(string_2.to_string(), a.get(&string_2.to_string()).unwrap() + 1);
            } else {
                a.insert(string_2.to_string(), 1);
            }
        } else {
            let mut hm = HashMap::<String, usize>::new();
            hm.insert(string_2.to_string(), 1);

            self.words.insert(string_1.to_string(), hm);
        }
    }

    pub fn print(&mut self) {
        for x in self.words.keys() {
            for y in self.words.get(x).unwrap().keys() {
                println!("{}, {}, {}", x, y, self.words.get(x).unwrap().get(y).unwrap());
            }
        }
    }

    //Private functions
    pub fn values_sum(&mut self) -> usize {
        let mut sum: usize = 0;
        for x in self.words.keys() {
            for y in self.words.get(x).unwrap().values() {
                sum += y;
            }
        }

        return sum;
    }

    pub fn next_word(&mut self, string: String) {

    }
}
