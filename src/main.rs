
extern crate clap;

use std::collections::HashMap;
use std::io::{Read, BufReader};
use clap::{App, Arg};
use std::fs::File;

fn main() {

    let matches = App::new("scotext")
        .version("0.1.1")
        .about("Scores input based on english language character frequency")
        .author("Gavyn Riebau")
        .arg(Arg::with_name("verbose")
             .help("Increases the verbosity of output")
             .long("verbose")
             .short("v")
             .takes_value(false))
        .arg(Arg::with_name("dictionary")
             .help("Path to a dictionary file with english language words. If one of the words in the dictionary is found in the input, the text score will be increased.")
             .long("dictionary")
             .short("w")
             .value_name("dictionary"))
        .get_matches();

    let character_scores = get_char_score_map();
    let mut score = 0.0;
    let mut stdin = std::io::stdin();
    let mut buffer = String::new();

    let _ = stdin.read_to_string(&mut buffer);
    let input = buffer.to_lowercase();

    if matches.is_present("verbose") {
        println!("Read input: {}", input);
    }

    // Iterate each character and increment "score" by the frequency score for that character.
    for c in input.chars() {
        if character_scores.contains_key(&c) {
            let char_score = character_scores.get(&c).unwrap();
            if matches.is_present("verbose") {
                println!("Char: {} - {}", c, char_score);
            }
            score = score + char_score;
        }
    }

    // Check if each word in the provided dictionary is present.
    if matches.is_present("dictionary") {
        let path = matches.value_of("dictionary").unwrap();
        match File::open(path) {
            Ok(file) => {
                // Will hold all the words in the dictionary file.
                let mut dictionary_lines : Vec<String> = Vec::new();

                // Read all the words and push them to the dictionary vector.
                let mut reader = BufReader::new(file);
                let mut dictionary_data = String::new();
                let _ = reader.read_to_string(&mut dictionary_data);

                for line in dictionary_data.lines() {
                    let word = line.to_lowercase();
                    dictionary_lines.push(word);
                }

                // Sort the dictionary in order of word length, largest words to smallest words.
                dictionary_lines.sort_by(|a, b| {
                    let x = a.len();
                    let y = b.len();

                    y.cmp(&x)
                });

                // Check if the input contains a word from the dictionary.
                // Each time a word is matched it's removed from the input so there isn't double
                // counting of words.
                //
                // For each word the score is increased by 3 * e ^ word_length.
                // In this way large words contribute exponentially more to the overall score.
                let mut cloned_input = input.clone();
                for word in dictionary_lines {
                    if cloned_input.contains(word.as_str()) {
                        if matches.is_present("verbose") {
                            println!("Matched word: {}", word);
                        }
                        let adjustment = 3.0 * (word.len() as f32).exp();
                        score = score + adjustment;

                        cloned_input = cloned_input.replacen(word.as_str(), "", 1);
                    }
                }
            },
            Err(err) => {
                println!("Failed to open dictionary file '{}' because: {:?}", path, err);
            }
        }
    }

    if matches.is_present("verbose") {
        println!("Character score was: {}", score);
    } else {
        println!("{:>8.2}", score);
    }
}

// Creates a dictionary where:
// key      - character
// value    - frequency score
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

