use dialoguer::{Select, Input, Password, Confirm};
use super::{Callframe, common};
use std::collections::HashMap;

pub fn edit_callframe(mut callframe: &mut Callframe) {
    /*
    Keeping track of the fields in a separate vec rather than using the key set of the 
    hashmap ensures that they render in the same order every time in the CLI
     */ 
    let fields = vec!["Name", "URL", "Method", "Headers", "Parameters"];
    let field_map: HashMap<&str, fn(&mut Callframe)> = HashMap::from([
        ("Name", edit_name as fn(&mut Callframe)),
        ("URL", edit_url as fn(&mut Callframe)),
        ("Method", edit_method as fn(&mut Callframe)),
        ("Headers", edit_headers as fn(&mut Callframe)),
        ("Parameters", edit_params as fn(&mut Callframe)),
    ]);

    let mut editing = true;
    while editing {
        let index = Select::new()
            .with_prompt("Select a field to edit")
            .items(&fields)
            .default(0)
            .interact()
            .unwrap();
        let field = &fields[index];

        if let Some(func) = field_map.get(field) {
            func(&mut callframe);
        }

        editing = Confirm::new()
            .with_prompt("Would you like to edit another field?")
            .interact()
            .unwrap();
    }
}

pub fn edit_name(callframe: &mut Callframe) {
    callframe.name = Input::new()
        .with_prompt("New name")
        .default(callframe.name.clone())
        .interact_text()
        .unwrap();
}

pub fn edit_url(callframe: &mut Callframe) {
    callframe.url = Input::new()
        .with_prompt("New URL")
        .default(callframe.url.clone())
        .interact_text()
        .unwrap();
}

pub fn edit_method(callframe: &mut Callframe) {
    callframe.method = common::select_method();
}

pub fn edit_headers(callframe: &mut Callframe) {
    println!("Placeholder for editing headers");
}

pub fn edit_params(callframe: &mut Callframe) {
    println!("Placeholder for editing params");
}
