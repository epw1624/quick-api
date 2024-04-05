use std::{fs, io};

pub fn get_filenames_from_directory(path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?;

    let file_names: Vec<String> = entries   
        .filter_map(|entry| {
            let path = entry.ok()?.path(); // ignore None values
            if path.is_file() { // ignore non-files
                path.file_name()?.to_str().map(|s| s.to_owned())
            }
            else {
                None
            }
        })
        .collect();

    Ok(file_names)
}