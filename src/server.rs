use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn practical_length(buffer: &[u8]) -> usize {
    /*
    Implementing some kind of strlen, finding the index of the first zero element.
    Assumptions: Buffers end at 0 (TcpStream::read probably doesn't make them zero, it's just the buffer initialization in user code)
                 Buffers don't include any 0 byte which isn't the end
                    - according to https://stackoverflow.com/questions/6907297/can-utf-8-contain-zero-byte, that's the case
                 Buffers aren't simply too long - in case no zero is found in the buffer, it will panic!
    */
    return buffer.iter()
                 .enumerate()
                 .find_map(|(index, byte)| -> Option<usize>
                    {
                        if *byte == 0u8 { Some(index) }
                        else { None }
                    })
                 .expect("Request too long");
}

fn handle(mut stream: TcpStream, application_handler: impl Fn(&str) -> String) {
    let mut ok = true;
    while ok {
        let mut request_buffer = [0u8; 1024];
        match stream.read(&mut request_buffer) {
            Ok(0) => break,
            Ok(len) => println!("Successfully read {} bytes", len),
            Err(e) => { println!("Error on reading: {}", e); ok = false; }
        }
        let length: usize = practical_length(&request_buffer);
        let request = std::str::from_utf8(&request_buffer[..length]).unwrap();
        
        let response = application_handler(request);
        stream.write_all(response.as_bytes()).unwrap_or_else(|e| { println!("Error on writing: {}", e); ok = false; });
        stream.flush().unwrap_or_else(|e| { println!("Error on flush: {}", e); ok = false; });
    }
    
    println!("Done!");
}

pub fn serve(application_handler: impl Fn(&str) -> String) {
    let listener = TcpListener::bind("127.0.0.1:5040").expect("An error occured while binding");

    for incoming_stream in listener.incoming() {
        match incoming_stream {
            Ok(valid_stream) => { 
                println!("Handling valid stream");
                handle(valid_stream, &application_handler);
            },
            Err(e) => println!("Error while handling incoming stream: {}", e)
        }
    }
}