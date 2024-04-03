use super::Callframe;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn load_callframe() -> Option<Callframe> {
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