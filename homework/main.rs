use std::io::{self, Write};


fn read_input() -> String {
    io::stdout()
	            .flush()
        .expect("Error flushing stdout.");
    let mut guess: String = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Error reading line.");
    guess
}



fn change_word_repr(word: &str, guess: char, word_repr: &mut String, guessed_letters: &mut String) {
    word.chars().enumerate().for_each(|(letter_ind, letter)| {
        if letter == guess {
            word_repr.replace_range(letter_ind..(letter_ind+1),
            &guess.to_string());
        }
    });
    guessed_letters.push_str(&guess.to_string());
}


fn main() {
    let word = "university";
    let mut word_repr = String::from("----------");
    let mut guessed_letters = String::from("");
    let mut attempts_left: i32 = 5;
    loop {
        println!("The word so far is {}", &word_repr);
        println!("You have guessed the following letters: {}", &guessed_letters);
        println!("You have {} guesses left", &attempts_left);
        print!("Please input your guess: ");
    let guess = read_input();
    
    let guess: char = match guess.trim().parse() {
        Ok(char) => char,
        Err(_) => continue,
    };

    if guessed_letters.contains(guess) {
        println!("You have already used this letter!");
        continue;
    }

    if word.contains(guess) {
        change_word_repr(word, guess, &mut word_repr, &mut guessed_letters);
        if !word_repr.contains("-") {
            println!("You win!");
            break;
        }
    }
    else {
        attempts_left -= 1;
        println!("There is no such letter in the word :(");
        if attempts_left == 0 {
            println!("You lost!");
            break
        }
    }
    }
}
