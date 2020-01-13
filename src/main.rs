mod utils;

use utils::filereader::*;
use utils::wordmap::*;
use utils::word_generator::*;

use structopt::StructOpt;
use std::fs::File;

// TODO: Add better error handling

#[derive(StructOpt)]
struct Cli {
    //Path to read the file from
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    amount: i32
}

fn main() {
    // Get arguments
    let args = Cli::from_args();

    // Create filereader from path
    let mut fileread = FileReader::new(&args.path);
    let mut wordmap = WordMap::new();

    // Read contents to wordmap
    fileread.read_to_wordmap(&mut wordmap);

    // Create a WordGenerator based on wordmap
    let wordgen = WordGenerator::new(wordmap);

    // Generate required amount of words
    println!("{}", wordgen.generate(args.amount));
}