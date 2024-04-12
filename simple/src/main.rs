use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<HashMap<usize, TcpStream>>>, id: usize) {
    let addr = stream.peer_addr().expect("Issue to get client address");
    println!("Client {} connected from {} ", id, addr);

    clients
        .lock()
        .unwrap()
        .insert(id, stream.try_clone().unwrap());

    loop {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnect", addr);
                clients.lock().unwrap().remove(&id);
                break;
            }
            Ok(n) => {
                let request_str = String::from_utf8_lossy(&buffer[..]);
                println!("Received message from {}: {}\n", id, request_str);
                let clients = clients.lock().unwrap();
                for (client_id, mut client_stream) in clients.iter() {
                    if *client_id != id {
                        client_stream
                            .write_all(&buffer[..n])
                            .expect("Unable to write to all clients");
                    }
                }
                // stream
                //     .write_all(&mut buffer)
                //     .expect("Error found while writing");
            }

            Err(e) => {
                eprintln!("Error reading from client {}: {}", addr, e);
                break;
            }
        }
        // stream
        //     .read(&mut buffer)
        //     .expect("Error while reading stream");
        // let request_str = String::from_utf8_lossy(&buffer[..]);
        // println!("Recieved request:\n{}", request_str);

        // let resp = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";
        // stream
        //     .write_all(resp.as_bytes())
        //     .expect("Error while writing all stream");
        // stream.flush().expect("Error to flush ");
    }
}

fn main() {
    println!("This is very nice!");

    let listener = TcpListener::bind("127.0.0.1:3000").expect("Error to bind");
    println!("Server listenning on port 3000...");

    let clients: Arc<Mutex<HashMap<usize, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut id_counter = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client_ref = Arc::clone(&clients);
                let id = id_counter;
                id_counter += 1;
                thread::spawn(move || {
                    handle_client(stream, client_ref, id);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
