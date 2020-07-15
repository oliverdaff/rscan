use clap::{App, Arg};
use futures::stream::{self, StreamExt};
use tokio::net::TcpStream;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
enum PortResult {
    Open(String, u16),
    Closed(String, u16),
}

impl PortResult {
    pub fn is_open(&self) -> bool {
        matches!(*self, PortResult::Open(_, _))
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
enum PortArg {
    Single(u16),
    Range(u16, u16),
}

#[tokio::main]
async fn main() {
    let command = App::new("rscan ")
        .version("0.1.0")
        .author("Oliver Daff")
        .about("A simple TCP port scanner")
        .arg(
            Arg::with_name("HOSTS")
                .help("The hosts to scan seperated by commas.")
                .required(true)
                .takes_value(true)
                .multiple(true)
                .value_delimiter(","),
        )
        .arg(
            Arg::with_name("PORTS")
                .long_help("The ports to scan, seperated by commas, ranges seperated by -")
                .help("The ports to scan.")
                .required(true)
                .short("p")
                .takes_value(true)
                .multiple(true)
                .value_delimiter(","),
        )
        .get_matches();

    let hosts = command.values_of("HOSTS").unwrap();
    let (ports_ranges, errors) = parse_ports(command.values_of("PORTS").unwrap().collect());

    if !errors.is_empty() {
        panic!("Input ports had errors: {:?}", errors);
    }

    let ports: Vec<u16> = ports_ranges
        .iter()
        .map(|pa| match pa {
            PortArg::Single(p) => (*p..=*p),
            PortArg::Range(s, e) => (*s..=*e),
        })
        .flat_map(|r| r.clone())
        .collect();

    let sockets = hosts.flat_map(|host| ports.iter().map(move |port| (host, port)));

    //let con_stream = stream::iter(sockets);
    let con_stream = stream::iter(sockets).map(|(host, port)| open_connection(host, *port));
    let results = con_stream.buffer_unordered(100).collect::<Vec<_>>().await;
    let (mut open, _): (Vec<_>, Vec<_>) = results.iter().partition(|x| PortResult::is_open(x));
    open.sort();

    println!("Open Ports:");
    open.iter().for_each(|pr| {
        if let PortResult::Open(h, p) = pr {
            println!("{}:{}", h, p)
        }
    });
}

fn parse_ports(ports: Vec<&str>) -> (Vec<PortArg>, Vec<String>) {
    let (ports, errors): (Vec<_>, Vec<_>) =
        ports.iter().map(|x| parse_port(x)).partition(Result::is_ok);
    (
        ports.into_iter().map(Result::unwrap).collect(),
        errors.into_iter().map(Result::unwrap_err).collect(),
    )
}

fn parse_port(port: &str) -> Result<PortArg, String> {
    if port.contains('-') {
        let parts: Vec<&str> = port.split('-').collect();
        if parts.len() == 2 {
            match (parts[0].parse::<u16>(), parts[1].parse::<u16>()) {
                (Ok(s), Ok(e)) if s < e => Ok(PortArg::Range(s, e)),
                (Ok(s), Ok(e)) if s == e => Ok(PortArg::Single(s)),
                _ => Err(format!("Error parsing port: {}", port)),
            }
        } else {
            Err(format!("Error parsing port: {}", port))
        }
    } else {
        port.parse::<u16>()
            .map(PortArg::Single)
            .map_err(|_| format!("Error parsing port {}", port))
    }
}

async fn open_connection(host: &str, port: u16) -> PortResult {
    TcpStream::connect(format!("{}:{}", host, port))
        .await
        .map_or(PortResult::Closed(host.to_string(), port), |_| {
            PortResult::Open(host.to_string(), port)
        })
}
