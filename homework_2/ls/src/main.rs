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
