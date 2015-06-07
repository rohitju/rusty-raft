extern crate protobuf;
mod vote;

use std::net::{TcpListener, TcpStream};
use std::thread;

use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    let mut buf;
    let mut vote = vote::VoteRequest::new();
    vote.set_candidateId("abcd".to_string());    
    loop {
        buf = [0; 512];
        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    // we've got an EOF
                    break;
                }
                m
            },
        };
        let mut strCpy = stream.try_clone().unwrap();
        let  coStream = ::protobuf::CodedOutputStream::new(&mut strCpy);

        /*match stream.write(vote.get_candidateId().as_bytes()) {
            Err(_) => break,
            Ok(_) => continue,
        }*/
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();
    println!("Started server on 127.0.0.1 at port 12345.");
    for stream in listener.incoming() {
        match stream { 
            Err(e) => { println!("Failed: {}", e)}
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
        }
    }
}
