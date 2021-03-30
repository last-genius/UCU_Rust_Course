use std::io;
use std::io::Write;

fn main() {
    // let max_word_len = 20;
    let word = "killme";
    let word_len = word.chars().count();
    let mut health = 5;

    println!("The secret word is: {}\n", word);
    // let mut lives = 5;
    let mut word_search = String::new();
    let mut correct_letters = String::new();
    for _ in 0..word_len {
        word_search.push_str("-")
    }
    loop {
        if health == 0 {
            println!("Ohh Gad, you suck and die ╰⋃╯\nOnly Jesus can help you now...");
            break;
        } else if word == word_search {
            println!("You win! Congratulations (ﾐ●ﻌ●ﾐ)ฅ");
            break;
        }
        println!("The word so far is {}", word_search.to_string());
        println!("You have guessed the following letters: {}", correct_letters.to_string());
        println!("You have {} guesses left", health);

        print!("Please guess a letter: ");
        io::stdout()
                .flush()
                .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
                .read_line(&mut guess)
                .expect("Error reading line.");
        guess = guess.to_string();
        guess.pop();

        if guess.len() != 1 {
            println!("Wrong input! Try to be better!!!");
            println!("As punishment you lose one heart\n");
            health -= 1;
            continue;
        }

        if word.contains(&guess) & !correct_letters.contains(&guess) {
            correct_letters.push_str(&guess);
            let mut word_search_new = String::new();
            let mut count_word = word.chars();
            let mut count_word_search = word_search.chars();
            for _ in 0..word_len {
                let char_word = count_word.next().unwrap();
                let char_word_search = count_word_search.next().unwrap();
                if char_word == guess.parse().unwrap() {
                    word_search_new.push_str(&guess);
                } else if char_word_search != '-' {
                    word_search_new.push_str(&char_word_search.to_string());
                } else {
                    word_search_new.push_str("-");
                }
            }
            word_search = word_search_new;
        } else {
            println!("Oh noo, you miss... Minus one heart :(");
            health -= 1;
        }
        println!();
    }
}
        // println!("{}", guess);
        // println!("{}", guess.chars().count());
        // // guess = guess.chars();
        // {
        //     for ch in guess.chars() {
        //         println!("--------------------");
        //         println!("{}", ch);
        //         println!("--------------------\n0");
        //     }
        //     // let mut guess = guess.to_str();
        // }
