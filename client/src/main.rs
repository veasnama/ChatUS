use std::fs::ReadDir;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
fn read_message(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Server disconnected");
                break;
            }
            Ok(_) => {
                // convert the received bytes to a stinrg and print
                let message = String::from_utf8_lossy(&buffer[..]);
                println!("Server says: {}", message);
            }

            Err(e) => {
                eprintln!("Errro reading from server: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let mut stream =
        TcpStream::connect("127.0.0.1:3000").expect("Found issue while connecting to server");

    // spwan a thread to read messages from the server
    let clone_stream = stream.try_clone().expect("Failed to clone steram");
    thread::spawn(move || {
        read_message(clone_stream);
    });

    // Main thread to send messages to the server
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Send user input to the server
        stream
            .write_all(input.trim().as_bytes())
            .expect("Failed to write all bytes");
    }
    // let request = "GET / HTTP/1.1\r\nHost: localhost:3000\r\n\r\n";

    // let response = String::new();
    // stream
    //     .write_all(request.as_bytes())
    //     .expect("Found issue while writing to server");
    // println!("Response:\n{}", response);
}
