mod wordlist;
mod filereader;

use crate::wordlist::WordList;
use crate::filereader::FileReader;

const PATH: &str = "src/text.txt";

fn main() {
    let mut file_reader = FileReader::new(PATH);
    let mut word_list = WordList::new();

    file_reader.read_to_wordlist(&mut word_list);

    word_list.print();
}