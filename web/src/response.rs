use std::{fs::File, io::Read};

pub fn handle_response(filename: String, status: String) -> String {
    let mut content = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let content_length = content.len();

    format!("{status}\r\nContent-Length: {content_length}\r\n\r\n{content}")
}
