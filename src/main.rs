
extern crate clap;
extern crate score_text;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::Read;
use clap::{App, Arg, ArgMatches};
use std::fs::File;

fn main() {

    env_logger::init().unwrap();

    let matches = App::new("scotext")
        .version("0.3.0")
        .about("Scores input based on english language character frequency.")
        .author("Gavyn Riebau")
        .arg(Arg::with_name("dictionary")
             .help("Path to a dictionary file with english language words.\nIf one of the words in the dictionary is found in the input, the text score will be increased.")
             .long("dictionary")
             .short("w")
             .value_name("dictionary"))
        .arg(Arg::with_name("input")
             .help("File to be used as input.\nIf not supplied, input will be taken from stdin.")
             .long("input")
             .short("i")
             .value_name("FILE"))
        .get_matches();

    let mut score = 0.0;
    let input = read_input(&matches);

    trace!("Read input: {}", input);

    // Iterate each character and increment "score" by the frequency score for that character.
    for c in input.chars() {
        let char_score = score_text::score_character(c);
        trace!("Char: {} - {}", c, char_score);
        score = score + char_score;
    }

    // Check if each word in the provided dictionary is present.
    if matches.is_present("dictionary") {
        let path = matches.value_of("dictionary").unwrap();
        let dictionary = score_text::load_dictionary(path);
        let word_score = score_text::score_words(input, dictionary);
        score = score + word_score;
    }

    debug!("Character score was: {}", score);
    println!("{:>8.2}", score);
}

fn read_input(matches : &ArgMatches) -> String {
    let mut input = String::new();

    if matches.is_present("input") {
        let mut file = File::open(matches.value_of("input").unwrap()).unwrap();
        let _ = file.read_to_string(&mut input);
    } else {
        let mut stdin = std::io::stdin();
        let _ = stdin.read_to_string(&mut input);
    }
    input.to_lowercase();

    input
}
