extern crate protobuf;
extern crate getopts;
extern crate mio;
extern crate ini;

use getopts::Options;
use std::env;
use std::thread;
use std::io::Read;
use std::net::SocketAddr;
use std::io::{stderr, Write};
use std::process::exit;
use protobuf::*;
use mio::*;
use mio::tcp::{TcpListener, TcpStream};
use std::error::Error;
use ini::Ini;


mod cluster;
mod server;
mod log;
mod message;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

/// Read server config opts.
/// TODO: This is not the right place for this logic.
fn read_config_file(config_file: &str) -> (String, String) {
    let conf = Ini::load_from_file(config_file).unwrap();
    let section = conf.section(Some("Server")).unwrap();
    let address = section.get("address").unwrap();
    let id = section.get("id").unwrap();
    return (id, address);
}

/// The main event loop for the Raft server.
/// Runs till the server shuts down or fails.
/// TODO: See if this can be broken up.
fn main_loop(args: Args) {
    let (id, address) = read_config_file(args.config_file);
    let addr: SocketAddr = address.parse().unwrap();
    // let addr: SocketAddr = get_listen_address(args.config_file.as_str()).parse().unwrap(); [UNSTABLE]
    let logpath = "/var/raft_log_".to_string() + &args.id[..];
    let mut server = server::Server::new(logpath.to_string(), args.is_bootstrap, address, id);

    //Event loop stuf
    const TIMEOUT: Token = Token(0);
    const LISTENER: Token = Token(1);
    let mut event_loop = EventLoop::new().unwrap();
    struct EventHandler(TcpListener, Timeout, server::Server);

    let time_out = event_loop.timeout_ms(5000, 5000).unwrap();
    let tcp_server = TcpListener::bind(&addr).unwrap();
    event_loop.register(&tcp_server, LISTENER).unwrap();
    println!("Event loop set up... Listening for incoming messages.");
    event_loop.run(&mut EventHandler(tcp_server, time_out, server)).unwrap();


    impl Handler for EventHandler {
        type Timeout = u32;
        type Message = ();

        fn readable(&mut self, event_loop: &mut EventLoop<EventHandler>, token: Token, _: ReadHint) {
            match token {
                LISTENER => {
                    let EventHandler(ref mut tcp_server, ref mut time_out, ref mut server) = *self;
                    //let res = event_loop.clear_timeout(*time_out);
                    //println!("Clearing timeout returned {}", res as i32);
                    //*time_out = event_loop.timeout_ms(5000, 5000).unwrap();
                    let (mut stream, mut addr) = tcp_server.accept().unwrap();
                    let mut inputStream = ::protobuf::CodedInputStream::new(&mut stream);
                    let mut received_message = match inputStream.read_message::<message::Message>() {
                        Ok(v) => { v },
                            Err(e) => {
                                println!("Failed to read Protobuf message from stream: {}.", e);
                                return;
                            }
                    };

                    let message_type = received_message.get_m_type();
                    match message_type {
                        request if request == message::Message_MessageType::V_REQUEST => {
                            //Process V_REQUEST
                        }
                        reply if reply == message::Message_MessageType::V_REPLY => {
                            //Process V_REPLY
                        }
                        config if config == message::Message_MessageType::CONFIG => {
                            println!("Received cluster config. Attempting to set it...");
                            server.set_cluster_config(received_message.get_config());
                        }
                        append if append == message::Message_MessageType::APPEND => {
                            //Process append entries
                        }
                        _ => {
                            println!("Unexpected protobuf message");
                        }
                    }  

                }
                _ => panic!("unexpected token"),
            }
        }

        fn timeout(&mut self, event_loop: &mut EventLoop<EventHandler>, timeout: u32) {
            let EventHandler(ref mut tcp_server, ref mut time_out, ref mut server) = *self;
            *time_out =  event_loop.timeout_ms(5000, 5000).unwrap();
            println!("Timeout for leader heartbeat passed.");
        }
    }

}

/*fn exit_with_error<E: Error>(error: E, code: i32) -> ! {
     let _ = writeln!(&mut stderr(), "{}", error);
     exit(code);
}*/

struct Args {
    is_bootstrap: bool,
    config_file: String
}

impl Args {
    fn from_env() -> Option<Args> {
        let args: Vec<String> = env::args().collect();

        let mut opts = Options::new();
        opts.optflag("b", "bootstrap", "Bootstrap a Raft cluster with this server.");
        opts.optflag("h", "help", "print this help menu");
        opts.reqopt("c", "config", "Absolute path to file containing server configuration.", "/var/raft/server.conf");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {println!("{}", f); exit(0);}
        };

        if matches.opt_present("h") {
            print_usage(&args[0].clone(), opts);
            return None;
        }

        Some(Args {
            is_bootstrap: matches.opt_present("b"),
            config_file: matches.opt_str("config").unwrap(),
        })
    }
}

fn main() {

    let args = Args::from_env();

    //Spawn thread for main event loop
    thread::spawn(move || {
            main_loop(args.unwrap());
     }).join();
}
