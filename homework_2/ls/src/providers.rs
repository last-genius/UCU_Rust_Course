pub mod file_provider {
    use std::path::Path;
    use crate::models::{FileModel, FileSize};
    use std::{io, fs};
    use std::io::Error;

    pub fn get_files_in_directory(directory: &Path) -> Result<Vec<FileModel>, Error> {
        let mut file_vector = Vec::new();
        // Iterate over entries in the directory
        for entry in fs::read_dir(directory)? {
            let entry = entry?;

            file_vector.push(parse_dir_entry(&entry.path().as_path())?);
        }
        Ok(file_vector)
    }

    pub fn parse_dir_entry(entry: &Path) -> io::Result<FileModel> {
        let name = {
            let os_file_name = entry.file_name()
                .expect("Could not read file name from a given path");
            String::from(os_file_name.to_string_lossy())
        };
        let metadata = entry.metadata()?;

        let is_hidden = name.starts_with(".");
        let is_directory = metadata.is_dir();
        let size: u64 = metadata.len();
        let is_ro = metadata.permissions().readonly();

        Ok(FileModel { name, is_hidden, is_directory, size: FileSize { size }, is_ro })
    }
}