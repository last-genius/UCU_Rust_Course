use std::io;
use std::fs;
use std::env;
use std::process;
use std::io::Write;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() != 2 {
        println!("Usage: {} dictionary", args[0]);
        process::exit(1);
    }
    // read the whole file
    let contents: String = fs::read_to_string(&args[1]).
                    expect("Couldn't read given dictionary!");
    let words: Vec<&str> = contents.split('\n').collect();
    let word: &str = words[rand::thread_rng().gen_range(0..words.len())];
    // println!("Guseed word: {}", word);
    let mut guessed: Vec<char> = Vec::new(); // assume that word is in ascii

    let mut attempts = 5;
    while attempts > 0 {

        // Word so far
        print!("The word so far is: ");
        io::stdout().flush().expect("Error flushing stdout.");

        let mut k = 0;
        for i in 0..word.len() {
            if guessed.contains(&(word.as_bytes()[i] as char)) {
                print!("{}", &(word.as_bytes()[i] as char));
                k += 1;
            } else {
                print!("-");
            }
        }
        println!();


        if k == word.len() {
            println!("You won! Congrats");
            process::exit(0);
        }


        // guessed letters
        print!("You have guessed the following letters: ");
        io::stdout().flush().expect("Error flushing stdout.");
        for i in 0..guessed.len() {
            print!("{}, ", &guessed[i]);
        }
        println!();


        // number of attempts left
        println!("You have {} guesses left", attempts);
        io::stdout().flush().expect("Error flushing stdout.");


        // new guess
        print!("Please guess a letter: ");
        io::stdout().flush().expect("Error flushing stdout.");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read input");

        // 2 because of \n
        if user_input.len() > 2 { 
            println!("Type only one letter!");
            attempts -= 1;
        } else {
            let stripped = user_input.trim_end_matches('\n');
            if word.contains(&stripped) && !guessed.contains(&(stripped.as_bytes()[0] as u8 as char)) {
                guessed.push(stripped.as_bytes()[0] as char);
            } else {
                attempts -= 1;
            }
        }

        println!("\n");

    }

    println!("You lost, looser");
    println!("Actual word: {}", word);

}
