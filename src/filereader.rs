use std::fs::File;
use crate::wordlist::WordList;
use std::io::Read;
use std::vec::Vec;

pub struct FileReader {
    file: File
}

impl FileReader {
    pub fn new(path: &str) -> FileReader {
        FileReader {
            file: File::open(path).unwrap(),
        }
    }

    pub fn read_to_wordlist(&mut self, wl: &mut WordList) {
        let mut buffer = String::new();
        self.file.read_to_string(&mut buffer);

        for sentence in buffer.split("\n") {
            let words: Vec<&str> = sentence.split(' ').collect();

            for x in 0..words.len() - 1 {
                wl.add(words[x].to_string(), words[x + 1].to_string());
            }
        }
    }

    pub fn read_to_console(&mut self) {
        let mut buffer = String::new();
        self.file.read_to_string(&mut buffer);

        println!("{}", buffer);
    }
}