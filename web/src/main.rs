use std::net::TcpListener;

mod client;
mod response;
mod threadpool;

use client::handle_client;
use threadpool::ThreadPool;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    let mut thpool = ThreadPool::new(Some(4));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let exec_handle_client = move || handle_client(stream).unwrap();
                thpool.execute(Box::from(exec_handle_client));
            }
            Err(e) => println!("error: {}", e),
        }
    }

    Ok(())
}
