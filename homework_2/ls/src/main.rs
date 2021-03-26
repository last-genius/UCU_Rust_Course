mod models;
mod providers;

use std::env;
use std::io;
use std::path::{Path};
use providers::file_provider;
use crate::models::FileModel;

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
        match file_provider::get_files_in_directory(start_path) {
            Ok(mut file_models) => {
                let start_path = start_path.to_string_lossy();
                sort_file_table(&mut file_models);
                print_file_table(start_path.as_ref(), &file_models);
            }
            Err(e) => eprintln!("Error reading files in directory: {}", e)
        }
    } else {
        // If the path provided is a single file, just print its name
        println!("{}", start_path.file_name().unwrap().to_string_lossy());
    }

    Ok(())
}

fn print_file_table(start_path: &str, file_table: &Vec<FileModel>) {
    println!("List of files in {}", start_path);
    println!("{:36} {:9}", "Name", "Size");

    for model in file_table {
        println!(
            "{:36} {:9}",
            format!("{}{}", model.name, if model.is_directory { "/" } else { "" }),
            model.size.to_human_str()
        )
    }
}

fn sort_file_table(file_table: &mut Vec<FileModel>) {
    file_table.sort_unstable_by(|a, b| a.is_directory
        .cmp(&b.is_directory)
        .reverse()
        .then(a.name.cmp(&b.name)));
}
