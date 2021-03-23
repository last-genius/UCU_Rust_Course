use std::io;
use rand::Rng;
fn main() {

    let word_vec = vec!["word", "computer", "history", "science", "space"];

    let random_index = rand::thread_rng().gen_range(0..4);

    let secret_word: String = word_vec[random_index].to_string();

    let count = secret_word.chars().count();

    let mut lives = 0;

    let mut guess_count = 0;

    let mut letters: String = "".to_owned();

    let mut main_vec = vec![1, 2];
    main_vec.clear();

    let secret_vec: Vec <_> = secret_word.split("").collect();

    loop{

        let mut hint = String::with_capacity(count);


        for position in 0..count + 1{
            if letters.contains(secret_vec[position]){
                if secret_vec[position] != ""{
                    hint.push_str(secret_vec[position]);
            }
            }else{
                let dash = "-";
                hint.push_str(dash);
            }
        }

        println!("The word so far is {}", hint);

        let mut guess = String::new();

        println!("Plese enter a letter: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess = guess.trim().to_string();

        if guess.chars().count() > 1{
            println!("You have to input only one letter");
            continue;
        }

        if secret_word.contains(&guess){

            if letters.contains(&guess){
                
                println!("You have already guessed this letter!");
            }else{

                let vec: Vec<&str> = secret_word.rmatches(&guess).collect();

                guess_count = guess_count + vec.len();

                letters.push_str(&guess);
                println!("You have guessed the following letters: {}", letters);
                
            }

            if guess_count == count{
                println!("You have guessed the word: {}", secret_word);
                break;
            }

        }else{
            println!("wrong");
            lives = lives + 1;
            if lives == 5{
                break;
            }
        }
        println!("You have {} attempts left.", 5 - lives);
    }
}
