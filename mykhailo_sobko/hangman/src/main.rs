use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the Linux distro or die!\n");

    let distros = [
        "manjaro".to_string(),
        "debian".to_string(),
        "ubuntu".to_string(),
        "fedora".to_string(),
        "gentoo".to_string(),
        "kali".to_string(),
        "mint".to_string(),
        "arch".to_string(),
        "void".to_string(),
    ];

    let secret_distro = distros.choose(&mut thread_rng());

    let secret_distro = secret_distro.unwrap();

    let mut guessed_letters = String::new();

    for _ in secret_distro.chars() {
        guessed_letters.push_str("-");
    }

    let mut guesses_left = 10;

    while guesses_left > 0 {
        println!("Guessed letters: {}", guessed_letters);
        println!("Guesses left: {}", guesses_left);
        println!("Enter your guess: ");

        io::stdout().flush().expect("Error flushing the stdout");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess = guess.trim();

        if secret_distro.contains(&guess) & !guessed_letters.contains(&guess) {
            println!("\nYou have guessed a letter!");

            let mut new_guessed_letters = String::new();

            for (distro_letter, guessed_letter) in
                secret_distro.chars().zip(guessed_letters.chars())
            {
                if guessed_letter.to_string() != "-" {
                    new_guessed_letters.push_str(&guessed_letter.to_string());
                } else if guess == distro_letter.to_string() {
                    new_guessed_letters.push_str(guess);
                } else {
                    new_guessed_letters.push_str("-");
                }
            }
            guessed_letters = new_guessed_letters;
        } else {
            println!("\nWrong letter!")
        }
        if &guessed_letters == secret_distro {
            println!("\nYou won the game!");
            println!("{} was the secret distro", secret_distro);

            break;
        }
        guesses_left -= 1;
    }
    if guesses_left == 0 {
        println!("\nYou died!");
        println!("{} was the secret distro", secret_distro);
    }
}
