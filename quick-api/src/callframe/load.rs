use super::Callframe;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use dialoguer::{Select};

mod file_handler;

pub fn load_callframe() -> Option<Callframe> {
    let options: io::Result<Vec<String>> = file_handler::get_filenames_from_directory("data");
    match options {
        Ok(file_names) => {
            let selected_file_index = Select::new()
                .with_prompt("Choose a saved API call to load")
                .items(&file_names)
                .default(0)
                .interact()
                .unwrap();

            let filename = &file_names[selected_file_index];

            match load_from_file(format!("{}{}", "data/", &filename).as_str()) {
                Ok(callframe) => return Some(callframe),
                Err(_) => return None
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    

    match load_from_file("data/beer.json") {
        Ok(callframe) => return Some(callframe),
        Err(_) => return None
    }
}

fn load_from_file(filepath: &str) -> Result<Callframe, Box<dyn Error>> {
    let mut file = File::open(filepath)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let callframe: Callframe = serde_json::from_str(&content)?;

    Ok(callframe)
}