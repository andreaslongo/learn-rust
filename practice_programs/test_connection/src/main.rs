use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;

use clap::Parser;

/// Simple program to test TCP connectivity
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// DNS name or IP address
    host: String,

    /// TCP port
    port: u32,

    /// Wait limit for each connection in seconds
    #[arg(short, long, default_value_t = 1)]
    timeout: u64,
}

fn main() {
    let args = Args::parse();

    let timeout_in_seconds = Duration::new(args.timeout, 0);
    let connection = format!("{}:{}", args.host, args.port);

    let addrs_iter = connection.to_socket_addrs().unwrap();
    for addr in addrs_iter {
        match TcpStream::connect_timeout(&addr, timeout_in_seconds) {
            Ok(_) => println!("OK :: {connection} :: {addr}"),
            Err(_) => println!("BAD :: {connection} :: {addr}"),
        }
    }
}
