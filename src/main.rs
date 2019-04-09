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
}