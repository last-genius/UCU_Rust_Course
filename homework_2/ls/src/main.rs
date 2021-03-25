mod models;

use std::env;
use std::fs;
use std::io;
use std::path::{Path};
use std::io::{Error};
use std::fs::DirEntry;
use crate::models::{FileModel, FileSize};

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
        match read_files_in_dir(start_path) {
            Ok(mut file_models) => {
                println!("List of files in {}", start_path.to_string_lossy());
                println!("{:36} {:9}", "Name", "Size");

                file_models.sort_by(|a, b| a.name.cmp(&b.name));
                for model in file_models {
                    println!("{:36} {:9}", model.name, model.size.to_human_str())
                }
            }
            Err(e) => eprintln!("Error reading files in directory: {}", e)
        }
    } else {
        // If the path provided is a single file, just print its name
        println!("{}", start_path.file_name().unwrap().to_string_lossy());
    }

    Ok(())
}

fn read_files_in_dir(directory: &Path) -> Result<Vec<FileModel>, Error> {
    let mut file_vector = Vec::new();
    // Iterate over entries in the directory
    for entry in fs::read_dir(directory)? {
        let entry = entry?;

        file_vector.push(parse_dir_entry(&entry)?);
    }
    Ok(file_vector)
}

fn parse_dir_entry(entry: &DirEntry) -> io::Result<FileModel> {
    let name = {
        let os_file_name = entry.file_name();
        String::from(os_file_name.to_string_lossy())
    };
    let metadata = entry.metadata()?;
    let size: u64 = metadata.len();
    let is_ro = metadata.permissions().readonly();

    Ok(FileModel { name, size: FileSize { size }, is_ro })
}
