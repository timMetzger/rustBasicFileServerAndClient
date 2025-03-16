use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use BasicWebServer::ThreadPool;

fn send_data(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);
    let mut http_request: Vec<_> = Vec::new();

    
    for line in buf_reader.lines() {
        match line{
            Ok(line) => {
                if !line.is_empty() {
                    http_request.push(line)
                }
                else{
                    break;
                }
            }
            Err(err) => {
                println!("{}", err);
                return;
            },
        }
    }
    
    println!("Request: {http_request:#?}");

    let file = std::fs::File::open("../../file.txt");
    match file{
        Ok(mut file) => {
            // Write response
            stream.write_all("HTTP/1.1 200 OK\r\n".as_bytes()).unwrap();
            
            // Create the header
            let mut header = String::new();
            header.push_str("file.txt\r\n");
            header.push_str(file.metadata().unwrap().len().to_string().as_str());
            header.push_str("\r\n");
            
            // Write the header
            stream.write_all(header.as_bytes()).unwrap();
            
            // Send the file
            std::io::copy(&mut file, &mut stream).unwrap();
        }
        Err(err) => {
            println!("{}", err);
            stream.write_all("HTTP/1.1 404 FILE NOT FOUND\r\n\r\n".as_bytes()).unwrap();
        }
    }
}


fn main() {
    let addr = String::from("127.0.0.1:8080");
    println!("Starting Listener on {}", &addr);

    let listener = TcpListener::bind(&addr);

    match listener {
        Ok(listener) => {
            let pool = ThreadPool::new(10);

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        pool.execute(|| {
                            send_data(stream);
                        })
                    }
                    Err(_) => {}
                }

                println!("Listening...");
            }
        }
        Err(e) => {
            println!("Failed to bind to port: {}", e);
        }
    }
}







