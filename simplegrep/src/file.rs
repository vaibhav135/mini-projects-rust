use std::{
    fs::File,
    io::{Read, Result},
};

pub fn read_file(filepath: &String) -> Result<String> {
    let mut file = File::open(filepath)?;
    let mut filecontent = String::new();
    file.read_to_string(&mut filecontent)?;

    Ok(filecontent)
}
