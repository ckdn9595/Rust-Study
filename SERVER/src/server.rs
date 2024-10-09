use std::{f32::consts::E, io::Read, net::TcpListener};
use crate::http::Request;
use std::convert::TryInto;
pub struct Server{
    addr: String,
}

impl Server{
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            
            match listener.accept() {
                Ok( (mut stream, addr) ) => {
                    let mut buffer:[u8; 1024] = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {  
                            //println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("parse error"),
                            }
                            
                        },
                        Err(e) => {panic!("failed to read from connection: {}", e)}
                    }

                },
                Err(e) => panic!("{}", (e)),
                
            }
            
            let res = listener.accept();

            if res.is_err() {
                continue;
            }

            let ( stream, addr ) = res.unwrap();

        }
    }
}