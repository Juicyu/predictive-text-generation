use std::fs::File;
use crate::wordmap::WordMap;
use std::io::{Read, BufReader, BufRead};
use std::vec::Vec;

pub struct FileReader {
    file: File
}

impl FileReader {
    //Create a new FileReader
    pub fn new(path: &std::path::PathBuf) -> FileReader {
        FileReader {
            file: File::open(path).expect("Could not read file"),
        }
    }

    //Read a file and insert its contents to a WordMap
    // NOTE: This is highly inefficient for memory as all file content is stored at the same time.
    // TODO: REWORK
    pub fn read_to_wordmap(&mut self, wl: &mut WordMap) {
        //Create a string to read the file in
        let mut buffer = String::new();
        self.file.read_to_string(&mut buffer).expect("Could not read file");

        //Split the string from newlines, then split again from whitespaces
        for sentence in buffer.split("\n") {
            let words: Vec<&str> = sentence.split(' ').collect();

            //Add words to the WordMap
            for x in 0..words.len() - 1 {
                wl.add(words[x].to_string(), words[x + 1].to_string());
            }
        }
    }
}