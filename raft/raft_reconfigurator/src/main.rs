extern crate protobuf;
extern crate getopts;
extern crate mio;

mod message;

use protobuf::*;
use std::env;
use std::process::exit;
use mio::*;
use mio::tcp::{TcpListener, TcpStream};
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.reqopt("l", "leader", "IP address and port of leader.", "LEADER_IP:LEADER_PORT");
    opts.reqopt("a", "addresses", "IP address and ports of machines in cluster 
                    (comma separated).", "IP:PORT,IP:PORT,...");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { println!("{}", f); exit(1); }
    };
    if matches.opt_present("h") {
        print_usage(&args[0].clone(), opts);
        return;
    }
    let mut leader = matches.opt_str("leader").unwrap();
    let mut addresses = matches.opt_str("addresses").unwrap();

    let mut split = addresses.split(",");
    let mut vec_addr: Vec<&str> = split.collect();
    let mut vec_addr_string: Vec<String> = Vec::new();
    for s in &vec_addr {
        vec_addr_string.push(s.to_string());
    }
    let addresses: protobuf::RepeatedField<String> = protobuf::RepeatedField::from_vec(vec_addr_string);

    let mut config = message::ClusterConfig::new();
    config.set_leader(leader.clone());
    config.set_machines(addresses);

    let mut message = message::Message::new();
    message.set_m_type(message::Message_MessageType::CONFIG) ;
    message.set_config(config);
    
    let mut sendStream = match TcpStream::connect(&leader[..]) {
        Ok(v) => { 
            println!("Established connection with leader. Attempting to send cluster config."); 
            v
        },
        Err(e) => {
            println!("Failed to establish connection to leader with error: {}.", e);
            exit(1);
        }
    };

    let mut pBufStream = ::protobuf::CodedOutputStream::new(&mut sendStream);
    let res: ProtobufResult<()> = pBufStream.write_message_no_tag::<message::Message>(&message);
    match res {
        Ok(v) => println!("Successfully sent cluster config to leader."),
        Err(e) => println!("Failed to send cluster config to leader with error {}.", e)
    }
    pBufStream.flush();
}
