use std::io;
use std::io::Write;
fn main(){
    println!("Welcome to the game! Your task is to guess the word!");
    let secret_word = "linuxclub".to_string();
    let counter = secret_word.chars().count();
    println!("The secret word is {}", secret_word);
    let mut guessed_letters = "".to_string();
    let mut num_of_guesses = 5;
    let mut present_word = "".to_string();
    for _ in 0..counter{
        present_word.push_str(&"-".to_string());
    }
    loop {
        println!("The word so far is {}", present_word);
        println!("You have guessed the following letters: {}", guessed_letters);
        println!("You have {} guesses left", num_of_guesses);
        print!("Please input your guess: ");
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading line.");
        guess = guess.trim().to_string();
        if (guess.chars().count() != 1) || (!guess.chars().nth(0).unwrap().is_alphabetic()){
            println!("Please enter a letter!\n\n");
            continue;
        };
        if guessed_letters.contains(&guess){
            println!("You tried this letter before, please enter another!\n\n");
            continue;
        };
        guessed_letters.push_str(&guess);
        if secret_word.contains(&guess){
            let mut making_word = "".to_string();
            let mut chars_secret = secret_word.chars();
            let mut chars_present = present_word.chars();
            for _ in 0..counter{
                if guess == chars_secret.next().unwrap().to_string(){
                    making_word.push_str(&guess);
                    chars_present.next();
                }else{
                    making_word.push_str(&chars_present.next().unwrap().to_string());
                }
            };
            present_word = making_word;
            if present_word.contains(&"-".to_string()){
                println!("\nYou've guessed a letter!");
            }else{
                println!("Hoorayy, you won the game! The word is {}", secret_word);
                break;
            }
        }else{
            num_of_guesses -= 1;
            println!("\nSorry, your guess was wrong!");
            if num_of_guesses == 0{
                println!("Oops, you loose. Please try one more time!");
                break;
            }
        }
    }
}