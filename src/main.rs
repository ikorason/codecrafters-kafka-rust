use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    println!("Server listening on port 9092");

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                if let Err(e) = handle_client(_stream) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // create a buffer to read the request
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                println!("Client disconnected");
                return Ok(());
            }

            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Received request: {}", request);

            // Response header v0 format:
            // message_size (4 bytes) + correlation_id (4 bytes)
            let message_size: i32 = 4;
            // hard-coded correlation_id of 7 as a 32-bit signed integer
            let correlation_id: i32 = 7;

            stream.write_all(&message_size.to_be_bytes())?;
            stream.write_all(&correlation_id.to_be_bytes())?;
            stream.flush()?;

            println!("Sent message_size: {message_size}, correlation_id: {correlation_id}");
        }
        Err(e) => {
            eprintln!("Error reading from stream: {}", e);
        }
    }

    Ok(())
}
