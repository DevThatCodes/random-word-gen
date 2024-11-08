
use std::env;

use rand::{thread_rng, Rng};

fn rand(max: i32) -> i32 {
    thread_rng().gen_range(1..=max)
}

fn main() {

    let args : Vec<String> = env::args().collect();
    let consonants = ["b","c","d","f","g","h","j","k","l","m","n","p","q","r","s","t","v","w","x","y","z"];
    let vowels = ["a","e","i","o","u"];
    let mut wordlen = rand(4) + 4;
    if args.len() > 1 {
        wordlen = args[1].clone().parse().unwrap();
    }
    let mut word = String::new();
    let mut i = 0;
    while i<wordlen {
        i += 1;
        if rand(2) == 1{
            word += vowels[(rand(vowels.len().try_into().unwrap())-1) as usize]
        } else {
            word += consonants[(rand(consonants.len().try_into().unwrap())-1) as usize]
        }
    }

    println!("{word} {wordlen}");
}
