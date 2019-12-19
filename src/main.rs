mod wordmap;
mod filereader;

use crate::wordmap::WordMap;
use crate::filereader::FileReader;

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

    println!("{}", wordmap.gen_sentence(5));




    /*


    println!("{}", word_map.gen_sentence(args.words));
    */
}