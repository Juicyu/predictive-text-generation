mod wordmap;
mod filereader;
mod word_generator;

use crate::wordmap::WordMap;
use crate::filereader::FileReader;
use crate::word_generator::WordGenerator;

use structopt::StructOpt;
use std::fs::File;


#[derive(StructOpt)]
struct Cli {
    //Path to read the file from
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    // Get arguments
    let args = Cli::from_args();

    // Create filereader from path
    let mut file_reader = FileReader::new(&args.path);
    let mut wordmap = WordMap::new();

    file_reader.read_to_wordmap(&mut wordmap);
    let wordgen = WordGenerator::new(wordmap);

    println!("{}", wordgen.generate(3));
    print!("Printed")
}