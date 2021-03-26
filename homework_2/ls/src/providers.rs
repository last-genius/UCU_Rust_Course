pub mod file_provider {
    use std::path::Path;
    use crate::models::{FileModel, FileSize};
    use std::{io, fs};
    use std::fs::DirEntry;
    use std::io::Error;

    pub fn get_files_in_directory(directory: &Path) -> Result<Vec<FileModel>, Error> {
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
        let is_directory = metadata.is_dir();
        let size: u64 = metadata.len();
        let is_ro = metadata.permissions().readonly();

        Ok(FileModel { name, is_directory, size: FileSize { size }, is_ro })
    }
}