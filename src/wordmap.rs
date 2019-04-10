use rand;

use std::collections::HashMap;
use core::borrow::{BorrowMut, Borrow};
use rand::Rng;

pub struct WordMap {
    words: HashMap<String, HashMap<String, usize>>,
}

impl WordMap {
    //Create a new WordMap
    pub fn new() -> WordMap {
        WordMap {
            words: HashMap::new()
        }
    }

    //Add a word to the WordMap

    //This part needs rework
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

    //Print the contents of the WordMap (Word 1, Word 2, # of times appeared in text)
    pub fn print(&mut self) {
        for x in self.words.keys() {
            for y in self.words.get(x).unwrap().keys() {
                println!("{}, {}, {}", x, y, self.words.get(x).unwrap().get(y).unwrap());
            }
        }
    }

    //Calculate the sum of values
    pub fn sum_of_appearances_f64(&mut self) -> f64 {
        let mut sum: f64 = 0.0;
        for x in self.words.keys() {
            for y in self.words.get(x).unwrap().values() {
                sum += *y as f64;
            }
        }

        return sum;
    }

    //Calculate next word
    pub fn generate_next_word(&mut self, string: String) -> Option<String> {
        let sum_of_values: f64 = self.sum_of_appearances_f64();
        let mut data: &HashMap<String, HashMap<String, usize>> = &self.words;
        let mut sum: f64 = 0.0;
        let rng: f64 = rand::thread_rng().gen();

        if data.contains_key(&string) {
            for x in data.get(&string).unwrap().keys() {
                let value = *data.get(&string).unwrap().get(x).unwrap();
                sum += (value as f64) / sum_of_values;
                if sum >= rng {
                    return Some(x.to_string());
                }
            }
        }
        return None;
    }
}
