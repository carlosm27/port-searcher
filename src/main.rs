use std::net::TcpListener;
use clap::Parser;



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    #[arg(short, long)]
    port: u16,

}



fn main() {
    let args = Args::parse();
    let port = args.port;


    let is_available = port_is_available(port);

    if  is_available {
        println!("Port: {} is available", port);
    } else {
        println!("Port: {} is NOT available", port)
    }

}



fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

