use super::response::handle_response;
use std::{
    env::current_dir,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let reader = BufReader::new(&stream);
    let request: Vec<_> = reader
        .lines()
        .map(|value| value.unwrap())
        .take_while(|val| !val.is_empty())
        .collect();

    println!("request:\n");
    for line in &request {
        println!("{line}");
    }

    let response = if request[0] == "GET / HTTP/1.1" {
        let filename = format!("{}/web/src/html/hello.html", current_dir()?.display());
        let status = String::from("HTTP/1.1 200 OK");
        handle_response(filename, status)
    } else {
        let filename = format!("{}/web/src/html/not-found.html", current_dir()?.display());
        let status = String::from("HTTP/1.1 404 Not Found");
        handle_response(filename, status)
    };

    stream.write_all(response.as_bytes()).unwrap();

    Ok(())
}
