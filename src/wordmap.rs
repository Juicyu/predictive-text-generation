use rand;

use std::collections::HashMap;
use core::borrow::{BorrowMut, Borrow};
use rand::Rng;
use rand::seq::IteratorRandom;

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

    // Return reference to words which follow after parameter
    pub fn followers(&self, word: &str) -> Option<&HashMap<String, usize>> {
        self.words.get(word)
    }

    // Pick a random word
    pub fn random_word(&self) -> String {
        if !(self.words.is_empty()) {
            self.words.keys().into_iter().choose(&mut rand::thread_rng()).unwrap().clone()
        } else {
            String::from("test")
        }
    }

    //Add a word to the WordMap, TODO: REWORK
    pub fn add(&mut self, first: String, second: String) {

        if self.words.contains_key(&first) {
            let a= self.words.get_mut(&first).unwrap();

            if a.contains_key(&string_2) {

                a.insert(second.to_string(), a.get(&second.to_string()).unwrap() + 1);
            } else {
                a.insert(second.to_string(), 1);
            }
        } else {
            let mut hashmap = HashMap::<String, usize>::new();
            hashmap.insert(second.to_string(), 1);

            self.words.insert(first.to_string(), hm);
        }
    }
}
