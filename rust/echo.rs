// Echo server in rust
//

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::net::tcp::TcpListener;
use std::io::{Acceptor,Listener};

fn main () {
    let address = SocketAddr {
        ip: Ipv4Addr(127, 0, 0, 1),
        port: 8080
    };

    let server = TcpListener::bind(address);
    let mut acceptor = server.listen();

    for mut stream in acceptor.incoming() {
        let msg = stream.read_to_end().unwrap();
        stream.write(msg);
    }

}
