mod wordmap;
mod filereader;

use crate::wordmap::WordMap;
use crate::filereader::FileReader;

const PATH: &str = "src/text.txt";

fn main() {
    let mut file_reader = FileReader::new(PATH);
    let mut word_map = WordMap::new();

    file_reader.read_to_wordmap(&mut word_map);

    word_map.print();

    println!();

    for x in 0..10 {
        println!("{}", word_map.generate_next_word("coffee".to_string()).unwrap());
    }
}