use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    println!("Server listening on port 9092");

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_client(_stream).unwrap();
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // hard-coded correlation_id of 7 as a 32-bit signed integer
    let correlation_id: i32 = 7;

    // convert to bytes (little-endian)
    let bytes = correlation_id.to_le_bytes();

    stream.write_all(&bytes)?;
    stream.flush()?;

    println!("Sent correlation_id: {correlation_id} as bytes: {bytes:?}");

    Ok(())
}
