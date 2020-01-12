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

        let mut words: Vec<String> = Vec::new();
        let mut latest = self.map.random_word();
        words.push(String::from(&latest));

        for _i in 1..amount {
            latest = self.generate_word(&latest);
            words.push(String::from(&latest));
        }

        words.join(" ")
    }

    fn generate_word(&self, word: &str) -> String {

        match self.map.followers(word) {
            Some(followers) => {
                let sum_of_appearances = followers.values().fold(0, |sum, &val| sum + val);
                followers.keys().nth(0).unwrap().clone()
            },
            None => {
                self.map.random_word().to_string()
            }
        }
    }
}