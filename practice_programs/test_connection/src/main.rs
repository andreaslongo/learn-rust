use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;

use clap::Parser;
use clap::ValueEnum;

/// A simple program to test TCP connectivity
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// DNS name or IP address
    host: String,

    /// TCP port number
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    port: Option<u16>,

    /// Protocol
    #[arg(short, long, conflicts_with("port"))]
    protocol: Option<Protocol>,

    /// Wait limit for each connection in seconds
    #[arg(short, long, default_value_t = 1)]
    timeout: u16,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Protocol {
    Http,
    Https,
    Mssql,
    Smb,
}

fn main() {
    let args = Args::parse();
    let timeout_in_seconds = Duration::new(args.timeout.into(), 0);
    let port: u16 = match args.protocol {
        Some(Protocol::Http) => 80,
        Some(Protocol::Https) => 443,
        Some(Protocol::Mssql) => 1433,
        Some(Protocol::Smb) => 445,
        None => args.port.unwrap_or(443),
    };

    let connection = format!("{}:{}", args.host, port);
    let addrs_iter = connection.to_socket_addrs().unwrap();
    for addr in addrs_iter {
        match TcpStream::connect_timeout(&addr, timeout_in_seconds) {
            Ok(_) => println!("OK :: {connection} :: {addr}"),
            Err(_) => println!("BAD :: {connection} :: {addr}"),
        }
    }
}
