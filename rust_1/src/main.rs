
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the word :)");

    let secret_word = "killme";
    let mut guesses = 5;
    let secret_vec: Vec<char> = secret_word.chars().collect();
    let mut guessed_vec: Vec<char> = vec!['-'; secret_vec.len()];

    println!("The secret word is: {}\n", secret_word);

    while guesses >= 0 {
        if guessed_vec == secret_vec{
            println!("You won! ily <3");
        }
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        // println!("{:?}", guessed_vec);
        // println!("{:?}", secret_vec);
        let letter = guess.chars().nth(0).unwrap();

        println!("You guessed: {}", letter);
        let mut count = 0;
        let mut i = 0;
        let slice = &secret_vec;
        for i in 0..secret_word.len() {
            if secret_vec[i].eq(&letter) {
                guessed_vec.remove(i);
                guessed_vec.insert(i, letter);
                count += 1;
            }

        }
        if count == 0 {
            println!("Nope :<");
            guesses -= 1;
        }
        println!("{:?}", guessed_vec);
        println!("{:?}", secret_vec);
    }
    println!("You lost :< \n the word was {}", secret_word);
}
