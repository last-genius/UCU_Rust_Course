# Homework 2

I don't really care about how you submit it, but sending a pull request
with a new directory homework/name should probably be okay.

This time you've learned even more Rust concepts:
* All of Rust's basic syntax and types
* Module system
* Test system
* Error handling stuff

These, along with the resources I provide down below
should be enough to complete this assignment. You
shouldn't have to import any more crates, or learn
a lot of new syntax. I am not limiting you though,
you should start getting familiar with Rust documentation,
just don't overwhelm yourself!

(Feel free to re-read the lecture and look at the code again (it's available at 
`src/examples/`). You can also just always ask me if you need help with anything!)

## `ls` and `tree` implementations

This assignment requires you to implement a program that will take some
command-line input and produce output, be it a list of directory entries
in a list-like fashion, or a list of every entry of everything below it
in a tree-like fashion.

#### 1. `ls`

You should start with a basic (and stupid!) snippet of code that takes
one command-line argument (the directory to list) and lists it out!

```rust
use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    // Collecting command line arguments
    let args: Vec<String> = env::args().collect();

    // Making sure there is a filepath provided
    assert_eq!(args.len(), 2);

    // Getting that filepath provided to us by the user
    let file_path = &args[1];
    let start_path = Path::new(file_path);

    // If the path provided to us is a directory, read its entries
    if start_path.is_dir() {

        // Iterate over entries in the directory
        for entry in fs::read_dir(start_path)? {
            let entry = entry?;

	    // If it is a valid entry, print its name, otherwise just skip it
            match entry.path().file_name() {
                Some(path) => println!("{}", path.to_string_lossy()),
                None => continue,
            };
        }
    } else {
        // If the path provided is a single file, just print its name
        println!("{}", start_path.file_name().unwrap().to_string_lossy());
    }

    Ok(())
}
```

This code purposefully does not handle most of the errors in a nice way,
and just outputs the filenames (not really what your `ls -lah` would do,
right?). Start with this little snippet, launch it several times and see
how it handles different cases:

```bash
# '--' allows you to separate arguments to cargo from arguments to your program
# '.' is just a current directory in *nix!
$ cargo run -- .
.git
Cargo.toml
Cargo.lock
.gitignore
src
target

$ cargo run -- src
main.rs
```

Then, experiment with this code, and read through several documentation pages
of the types, functions and modules you will be working with, including
but not limited to:

* [std::path::Path](https://doc.rust-lang.org/std/path/struct.Path.html)
	* [std::path::PathBuf.to_string_lossy()](https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.to_string_lossy)
* [std::io](https://doc.rust-lang.org/std/io/index.html)
* [std::fs](https://doc.rust-lang.org/std/fs/index.html)
	* [std::fs::read_dir](https://doc.rust-lang.org/std/fs/fn.read_dir.html)
	* [std::fs::DirEntry](https://doc.rust-lang.org/std/fs/struct.DirEntry.html)

Feel free to modify the functionality of the program! For example, you might 
happen to see some methods and functions that would help you output the 
directory listing in a nicer way:

```
136 F Cargo.lock
233 D Cargo.toml
    D src
```

Or, you could try hiding the files starting with `.` and showing them
if the user provides an option! You could also look into [`clap`](https://docs.rs/clap/2.33.3/clap/)
and [its page in Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html)
to better handle user input and options.

*Note: this is the only external dependency you might need in this project, don't
forget to add it to `Cargo.toml`!*


Also, improve error handling with the methods we've discussed in the lecture,
and read up on more of them in the documentation:

* [std::result::Result](https://doc.rust-lang.org/std/result/enum.Result.html)
* [std::option::Option](https://doc.rust-lang.org/std/option/enum.Option.html)

I'm not asking anything specific from you here, but don't think that you
should leave this code as it is! It's necessary to customize the output
on your own, maybe branch it out into a different function. It's also
necessary to provide more feedback on invalid user input and other
possible errors!

*Documentation is your friend here, don't be afraid of looking into it.*

#### 2. `tree` 

After adding some more data on the entries in the directory listing,
you can create a new project and copy over the existing code there,
or just keep the `ls` code in a different module/function.

In this new project, you are going to have to implement basic `tree`
functionality, which is going to require you to recursively
go through the directories below the one specified by the user.

A simple output would look like this:

```
current_directory_name
|---Cargo.lock
|---Cargo.toml
|---src
    |---main.src
```

Feel free to experiment (once again!), although you probably are going to
have to split the argument parsing functionality and directory recursing functionality
into different functions for sure!

#### Bonus: `find`

If the previous assignments weren't difficult for you, or you just want more,
try implementing a `find` alternative, which searches directories for entries
that satisfy given regex patterns. You can add this functionality to your `ls`
module, or your `tree` module (or both)!

Look into this [regex crate](https://docs.rs/regex/1.4.2/regex/index.html) and 
specifically its is_match method! Once again, feel free to add user command line
options and use `clap` (see above) to write a more detailed help page!
