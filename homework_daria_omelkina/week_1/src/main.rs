use std::io;
use std::io::Write;

fn main() {
    let mut number_of_guesses = 5;
    let word = "killme";
    let letters: Vec<char> = word.chars().collect();
    let mut hidden_word: Vec<char> = "-".repeat(letters.len()).chars().collect();
    let mut guessed_letters = String::new();
    
    loop {
        if number_of_guesses == 0 {
            println!("Sorry to tell You, but that is a slightly incorrect choice and you've run out of guesses.\nHere, take Your talon.");
            break;
        }
        let s: String = hidden_word.iter().collect();
        println!("The word so far is {}", s);
        println!("You have guessed the following letters: {}", guessed_letters);
        println!("You have {} guesses left", number_of_guesses);
        print!("Please input your guess: ");

        io::stdout()
            .flush()
            .expect("Error flushing stdout.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        let mut reverse_counter = letters.len();
        let mut flag = true;

        while reverse_counter != 0 {
            if letters[reverse_counter - 1] == guess.chars().nth(0).unwrap() {
                hidden_word[reverse_counter - 1] = letters[reverse_counter - 1];
                guessed_letters += &*letters[reverse_counter - 1].to_string();
                flag = false;
            }
            reverse_counter -= 1;
        }
        if letters == hidden_word {
            println!("You are absolutely right.\nThe word is '{}'.\nYou won a beer.", word);
            break;
        }
        if flag {
            number_of_guesses -= 1;
        }
    }
}
