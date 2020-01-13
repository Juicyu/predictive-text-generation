use rand::{Rng, RngCore};

use std::vec::Vec;
use std::iter::FromIterator;

use super::wordmap::WordMap;

pub struct WordGenerator {
    map: WordMap,
}

impl WordGenerator {
    pub fn new(map: WordMap) -> WordGenerator {
        WordGenerator { map }
    }

    // Generate a new sentence
    pub fn generate(&self, amount: i32) -> String {
        let mut words: Vec<String> = Vec::new();

        let mut latest = self.map.random_word();
        words.push(String::from(&latest));

        for _i in 1..amount {
            latest = self.generate_word(&latest);
            words.push(String::from(&latest));
        }

        words.join(" ")
    }

    // Generate next word
    fn generate_word(&self, word: &str) -> String {

        match self.map.followers(word) {
            Some(followers) => {
                // TODO: IMPLEMENT PROPERLY
                let sum_of_appearances = followers.values().fold(0, |sum, &val| sum + val);
                let random_number = rand::thread_rng().gen_range(0, sum_of_appearances);
                let mut sum: usize = 0;
                for word in followers.keys() {
                    if followers.get(word).unwrap() >= &sum {
                        return word.clone();
                    } else {
                        sum += *followers.get(word).unwrap();
                    }
                }

                followers.keys().nth(0).unwrap().clone()
            },
            None => {
                self.map.random_word().to_string()
            }
        }
    }
}