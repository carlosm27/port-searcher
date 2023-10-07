use std::net::TcpListener;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    #[arg(num_args(0..), short, long)]
    ports: Vec<u16>,
}


fn main() {
    let args = Args::parse();
    let ports = args.ports;

    ports_available(ports.to_vec());

}

fn ports_available(ports: Vec<u16>) {
    for port in ports {
        if port_is_available(port) {
            println!("port {} is available", port);
        } else {
            println!("Port: {} is NOT available", port)
        }
    }
}


fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

