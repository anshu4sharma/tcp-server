use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to run the listener");
    println!("Server listening on port 8000");
    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {
                // this will run spawn a new thread and run handle_client
                std::thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                eprintln!("Failed to establish connection : {}", err);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    // this is a buffer to read data from the client
    let mut buffer = [0; 1024];

    // this line read data from the stream and stores it into buffer
    stream.read(&mut buffer).expect("failed to read the data");
    // this line convert the data in the buffer into a UTF-8 encoded string
    let request = String::from_utf8_lossy(&buffer[..]);

    println!("Recieved request : {}", request);

    let response = "Hello client".as_bytes();

    stream.write(response).expect("failed to write response");
}
