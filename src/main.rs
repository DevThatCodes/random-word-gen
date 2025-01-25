
use std::{env, time::{Instant, Duration}};


use rand::{thread_rng, Rng};

fn rand(max: i32) -> i32 {
    thread_rng().gen_range(1..=max)
}

#[derive(Debug, PartialEq)]
enum LetterTypes {
    Consonant,
    Vowel,
    None
}

fn generate_word(wordlen: i32, max_vowels_in_a_row: i32, max_consonants_in_a_row: i32, consonant_mutation_chance: i32, vowel_mutation_chance: i32) -> (String, Duration)  {
    // variables related to word making
    let consonants = ["b","c","d","f","g","h","j","k","l","m","n","p","q","r","s","t","v","w","x","y","z", "r", "s", "t", "l", "n"];
    let vowels = ["a","e","i","o","u"];
    let mut word = String::new();
    let mut i = 0;
    let mut last_letter_type : LetterTypes = LetterTypes::None;
    let mut streak : i32 = 1; // this is for same letter type streaks
    let max_consonants_in_a_row = max_consonants_in_a_row - 1;
    let max_vowels_in_a_row = max_vowels_in_a_row - 1;
    // word making
    let word_making_bench = Instant::now();
    while i<wordlen {
        i += 1;

        // starting letter
        if last_letter_type == LetterTypes::None {
            if rand(2) == 1 {
                word += "V";
                last_letter_type = LetterTypes::Vowel;
            } else {
                word += "C";
                last_letter_type = LetterTypes::Consonant;
            }
            continue;
        }

        if last_letter_type == LetterTypes::Vowel {

            if streak == max_vowels_in_a_row {
                streak = 0;
                word += "C";
                last_letter_type = LetterTypes::Consonant;
                continue;
            }

            if rand(consonant_mutation_chance) == 1 {
                streak = 0;
                word += "C";
                last_letter_type = LetterTypes::Consonant;
            } else {
                streak += 1;
                word += "V";
            }
            continue;
        }

        if last_letter_type == LetterTypes::Consonant {

            if streak == max_consonants_in_a_row {
                streak = 0;
                word += "V";
                last_letter_type = LetterTypes::Vowel;
                continue;
            }

            if rand(vowel_mutation_chance) == 1 {
                streak = 0;
                word += "V";
                last_letter_type = LetterTypes::Vowel;
            } else {
                streak += 1;
                word += "C";
            }
            continue;
        }
    }
    let word_making_bench_time = word_making_bench.elapsed();


    // replacing C with a consonant and V with a vowel 
    let replace_bench = Instant::now();
    let mut result_word = String::new();
    word.chars().for_each(|char| {
        if char == 'C' {
            result_word += consonants[(rand(consonants.len().try_into().unwrap())-1) as usize];
        } else if char == 'V' {
            result_word += vowels[(rand(vowels.len().try_into().unwrap())-1) as usize];
        }
    });
    let replace_bench_time = replace_bench.elapsed();
    println!("word_making_bench_time: {:.2?}, replace_bench_time: {:.2?}, total_time: {:.2?}", word_making_bench_time, replace_bench_time, word_making_bench_time + replace_bench_time);
    (result_word, word_making_bench_time+replace_bench_time)
}

fn main() {

    // variables
    let args : Vec<String> = env::args().collect();
    
    let mut wordlen = rand(4) + 4;
    if args.len() > 1 {
        wordlen = args[1].clone().parse().unwrap();
    }
    let mut target_word : String = String::new();
    if args.len() > 2 {
        target_word = args[2].clone().to_string();
    }
    println!("{target_word}");
    
    // word settings
    let max_vowels_in_a_row = 2;
    let max_consonants_in_a_row = 3;
    let consonant_mutation_chance = 4; // chance for something to become a consonant ( 1 in ? )
    let vowel_mutation_chance = 3; // chance for something to become a vowel ( 1 in ? )
    
    let mut result : (String, Duration) = (String::new(), Duration::new(0, 0));
    let mut time_taken : Duration = Duration::new(0, 0);

    if !target_word.is_empty() {
        // start timer
        let mut tries = 0;
        while result.0 != target_word {
            result = generate_word(wordlen, max_vowels_in_a_row, max_consonants_in_a_row, consonant_mutation_chance, vowel_mutation_chance);
            print!("{} | ", result.0);
            tries += 1;
            time_taken += result.1;
        }
        println!("\n\n Got target word \'{}\' in {tries} tries, which took {:.2?}!\n", result.0, time_taken);
    } else {
        result = generate_word(wordlen, max_vowels_in_a_row, max_consonants_in_a_row, consonant_mutation_chance, vowel_mutation_chance); 
    }

    println!("{} {wordlen}", result.0);
}

