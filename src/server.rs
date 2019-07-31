use super::handler;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn read_request(stream: &mut TcpStream) -> Result<String, String> {
    let mut request_buffer = [0u8; 1024];
    match stream.read(&mut request_buffer) {
        Ok(0) => Err(String::from("End of stream")),
        Ok(len) => {
            println!("Successfully read {} bytes", len);
            Ok(std::str::from_utf8(&request_buffer[..len]).unwrap().to_string())
        },
        Err(e) => Err(e.to_string())
    }
}

fn write_response(stream: &mut TcpStream, response: &String) -> std::io::Result<()> {
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn serve_single(mut stream: TcpStream) {
    loop {
        let request = match read_request(&mut stream) {
            Err(e) => { println!("Error while reading: {}", e); break; },
            Ok(req) => req
        };
        
        let response = handler::handle_raw(request);

        match write_response(&mut stream, &response) {
            Err(e) => { println!("Error while writing: {}", e); break; },
            _ => {}
        }
    }
    
    println!("Done!");
}

pub fn serve_incoming() {
    let listener = TcpListener::bind("127.0.0.1:5040").unwrap_or_else(|e| panic!("Error while binding: {}", e));

    for incoming_stream in listener.incoming() {
        match incoming_stream {
            Ok(valid_stream) => { 
                println!("Handling valid stream");
                thread::spawn(|| serve_single(valid_stream));
            },
            Err(e) => println!("Error while handling incoming stream: {}", e)
        }
    }
}