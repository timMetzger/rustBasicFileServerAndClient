use std::io::BufReader;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080");

    match stream {
        Ok(mut stream) => {
            stream.write(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();

            let mut buf_reader = BufReader::new(&stream);
            let mut response = String::new();
            let res = buf_reader.read_line(&mut response);
            match res{
                Ok(n) => {
                    if n > 0 && response != "HTTP/1.1 200 OK\r\n"{
                        println!("Server Response: {}", response);
                        return;
                    }
                }
                Err(e) => {
                    println!("Failed read: {:?}",e);
                    return
                }
            }
            
            let mut fileName = String::new();
            let mut fileSizeStr = String::new();
            buf_reader.read_line(&mut fileName).expect("Could read not file name");
            buf_reader.read_line(&mut fileSizeStr).expect("Could read not file size");

            fileName = fileName.trim().to_owned();
            fileSizeStr = fileSizeStr.trim().to_owned();
            
            let mut fileSize = fileSizeStr.parse::<i32>().expect("Could not parse size");


            let mut f = std::fs::File::options().write(true).create(true).open(format!("C:/Debug/{fileName}"));

            match f{
                Ok(mut f) => {
                    // Read the remaining lines in the buffer and ensure bytes sum to file size
                    let mut bytesReceived : i32 = 0;
                    
                    let mut fileContents : Vec<u8> = Vec::new();
                   
                    match buf_reader.read_to_end(&mut fileContents){
                        Ok(n) => {
                            bytesReceived += n as i32;

                            if bytesReceived == fileSize {
                                f.write_all(&fileContents).unwrap();
                                println!("File Downloaded!");
                            }
                            else{
                                println!("Expected {:?} bytes, Found {:?} ", fileSize, bytesReceived);
                                return
                            }
                            
                            
                        }
                        Err(e) => {
                            println!("Failed read: {:?}",e);
                        }
                    }



                }
                Err(e) => {
                    println!("Error creating file: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Error with stream: {:?}", e);
        }
    }
}