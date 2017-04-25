
extern crate clap;

use std::collections::HashMap;
use std::io::Read;
use clap::{App, Arg};

fn main() {

    let matches = App::new("scotext")
        .version("0.1.0")
        .about("Scores input based on english language character frequency")
        .author("Gavyn Riebau")
        .arg(Arg::with_name("verbose")
             .help("Increases the verbosity of output")
             .long("verbose")
             .short("v")
             .takes_value(false))
        .get_matches();

    let mut character_scores = get_char_score_map();
    let mut score = 0.0;
    let mut stdin = std::io::stdin();
    let mut buffer = String::new();

    let _ = stdin.read_to_string(&mut buffer);
    let input = buffer.to_lowercase();

    if matches.is_present("verbose") {
        println!("Read input: {}", input);
    }

    for c in input.chars() {
        if character_scores.contains_key(&c) {
            let char_score = character_scores.get(&c).unwrap();
            if matches.is_present("verbose") {
                println!("Char: {} - {}", c, char_score);
            }
            score = score + char_score;
        }
    }

    if matches.is_present("verbose") {
        println!("Character score was: {}", score);
    } else {
        println!("{}", score);
    }
}

fn get_char_score_map() -> HashMap<char, f32> {
    let mut character_scores = HashMap::new();

    character_scores.insert('e', 12.702);	
    character_scores.insert('t', 9.056);	
    character_scores.insert('a', 8.167);	
    character_scores.insert('o', 7.507);	
    character_scores.insert('i', 6.966);	
    character_scores.insert('n', 6.749);	
    character_scores.insert('s', 6.327);	
    character_scores.insert('h', 6.094);	
    character_scores.insert('r', 5.987);	
    character_scores.insert('d', 4.253);	
    character_scores.insert('l', 4.025);	
    character_scores.insert('c', 2.782);	
    character_scores.insert('u', 2.758);	
    character_scores.insert('m', 2.406);	
    character_scores.insert('w', 2.360);	
    character_scores.insert('f', 2.228);	
    character_scores.insert('g', 2.015);	
    character_scores.insert('y', 1.974);	
    character_scores.insert('p', 1.929);	
    character_scores.insert('b', 1.492);	
    character_scores.insert('v', 0.978);	
    character_scores.insert('k', 0.772);	
    character_scores.insert('j', 0.153);	
    character_scores.insert('x', 0.150);	
    character_scores.insert('q', 0.095);	
    character_scores.insert('z', 0.074);	

    character_scores
}
