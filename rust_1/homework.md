This is a rough homework outline for the first week of the Rust course.

I don't really care about how you submit it, but sending a pull request
with a new directory homework/name should probably be okay.

We have already learned about a few main concepts which should help you here:
* Basic Rust syntax and types
* The concepts of ownership and borrowing
* Basic cargo workflow
* Basic error handling with .expect() and match expressions

(Feel free to re-read the lecture, I've updated it a little, and also
deleted unnecessary code snippets so it'd be easier to navigate the code 
directory. You can also just always ask me if you need help with anything!)

#### The assignment itself is super simple - implement a hangman-like terminal game!

An example of the status printout might look like this:

```
The word so far is ------
You have guessed the following letters:
You have 5 guesses left
Please guess a letter: l

You've guessed a letter!
The word so far is --ll--
You have guessed the following letters: l
You have 5 guesses left
Please guess a letter: k

You've guessed a letter!
The word so far is k-ll--
You have guessed the following letters: lk
You have 5 guesses left
Please guess a letter: m

You've guessed a letter!
The word so far is k-llm-
You have guessed the following letters: lkm
You have 5 guesses left
Please guess a letter: i

You've guessed a letter!
The word so far is killm-
You have guessed the following letters: lkmi
You have 5 guesses left
Please guess a letter: ...
```

And so on, until either you guess the word correctly, 
or you run out of your guess attempts (you might wanna consider
printinhg out a congratulatory message or something).

A few things you should keep in mind while doing this:
* Variables are immutable by default! 
If you need to mark them as mutable, use the `mut` keyword, as in: `let mut counter = 0`

* You can compare Strings just using `==`

* Don't forget Rust's format output: `println!("Look at this variable: {}", some_variable);`

* To read input from the user, you can write something like this:
```
print!("Please input your guess: ");

// This is a little technical thing, don't worry about it.
// Essentially, it just tells the compiler to make sure
// the prompt text is displayed before we ask for any input.
io::stdout()
	.flush()
        .expect("Error flushing stdout.");

let mut guess = String::new();

io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line.");
```

Don't overdo it! It's easy to get carried away and start googling
how to do this or that. You really don't need any new concepts,
and if you find yourself in such a screwup, just ask me, I'll try to help!

Please don't be mad at me or the compiler. We are really trying to help,
and if I am mistaken like 70% of the time, compiler never is, so you
should listen to it!
