
extern crate clap;
extern crate xor_utils;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::Read;
use clap::{App, Arg, ArgMatches};
use std::fs::File;
use xor_utils::{Score, ScoreAgainstDictionary};

fn main() {

    env_logger::init().unwrap();

    let matches = App::new("scotext")
        .version("0.5.0")
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
    score = score + input.score();

    // Check if each word in the provided dictionary is present.
    if matches.is_present("dictionary") {
        let path = matches.value_of("dictionary").unwrap();
        let dictionary = xor_utils::load_words_list(path);
        score = score + input.score_with_words(dictionary);
    }

    debug!("Character score was: {}", score);
    println!("{:>8.8}", score);
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
