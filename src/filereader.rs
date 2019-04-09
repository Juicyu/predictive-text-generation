use std::fs::File;
use crate::wordmap::WordMap;
use std::io::Read;
use std::vec::Vec;

pub struct FileReader {
    file: File
}

impl FileReader {
    //Create a new FileReader
    pub fn new(path: &str) -> FileReader {
        FileReader {
            file: File::open(path).unwrap(),
        }
    }

    //Read a file and insert its contents to a WordMap
    pub fn read_to_wordmap(&mut self, wl: &mut WordMap) {
        //Create a string to read the file in
        let mut buffer = String::new();
        self.file.read_to_string(&mut buffer);

        //Split the string from newlines, then split again from whitespaces
        for sentence in buffer.split("\n") {
            let words: Vec<&str> = sentence.split(' ').collect();

            //Add words to the WordMap
            for x in 0..words.len() - 1 {
                wl.add(words[x].to_string(), words[x + 1].to_string());
            }
        }
    }

    //Print text to console from file
    pub fn read_to_console(&mut self) {
        let mut buffer = String::new();
        self.file.read_to_string(&mut buffer);

        println!("{}", buffer);
    }
}