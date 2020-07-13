use futures::stream::{self, StreamExt};
use tokio::net::TcpStream;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
enum PortResult {
    Open(u16),
    Closed(u16)
}

impl PortResult {
    pub fn is_open(&self) -> bool {
        matches!(*self, PortResult::Open(_))
    }

}

#[tokio::main]
async fn main() {
    let x = stream::iter(1u16..100).map(|p| open_connection(p));
    let results = x.buffer_unordered(100).collect::<Vec<_>>().await;
    let (mut open, _): (Vec<_>, Vec<_>) = results.iter().partition(|x|PortResult::is_open(x));

    open.sort();

    println!("Open Ports:");
    for port in open {
        match port {
            PortResult::Open(p) => println!("{}", p),
            _ => ()
        }
    }
}

async fn open_connection(port: u16) -> PortResult {
    TcpStream::connect(format!("scanme.nmap.org:{}", port)).await
    .map_or(PortResult::Closed(port), |_| PortResult::Open(port))

}