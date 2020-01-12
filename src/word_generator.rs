use rand::{Rng, RngCore};

use crate::wordmap::*;

use std::vec::Vec;
use std::iter::FromIterator;

pub struct WordGenerator {
    map: WordMap,
}

impl WordGenerator {
    pub fn new(map: WordMap) -> WordGenerator {
        WordGenerator { map }
    }

    // Generate a new sentence with a maximum of <amount> words
    pub fn generate(&self, amount: i32) -> String {
        // TODO: USE WORDMAP TO GENERATE A NEW SENTENCE

        let mut words: Vec<String> = Vec::new();
        let mut latest = self.map.random_word();

        for _i in 0..amount {
            latest = self.generate_word(&latest);
            words.push(String::from(&latest));
        }

        words.iter().map(|word| word.to_string()).collect()
    }

    fn generate_word(&self, word: &str) -> String {
        let mut new_word= String::new();

        match self.map.followers(word) {
            Some(followers) => {
                let random = (followers.values().fold(0, |sum, &val| sum + val)) * rand::thread_rng().next_u32() as usize;
                let mut sum: usize = 0;

                for x in followers.keys() {
                    sum += *followers.get(x).unwrap();
                    if sum >= random {
                        new_word = x.to_string();
                    }
                }

            },
            None => {
                new_word = self.map.random_word().to_string();
            }
        }

        new_word
    }
}