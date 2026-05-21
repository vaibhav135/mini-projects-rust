use std::fs::File;
use std::io::{Read, Result};

pub fn read_file(filepath: &String) -> Result<String> {
    let mut file = File::open(filepath)?;
    let mut filecontent = String::new();
    file.read_to_string(&mut filecontent)?;

    Ok(filecontent)
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env::current_dir;

    #[test]
    fn test_valid_filepath() {
        let cwd = current_dir().unwrap();

        let filepath = format!("{}/assets/testfile.txt", cwd.display());
        let result = read_file(&filepath);
        let content = String::from(
            "I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
",
        );

        assert_eq!(result.unwrap(), content);
    }

    #[test]
    fn test_invalid_filepath() {
        let filepath = String::from("path/file1.txt");
        let result = read_file(&filepath);
        assert_eq!(
            result.unwrap_err().to_string(),
            "No such file or directory (os error 2)"
        );
    }
}
