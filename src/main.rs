use std::net::TcpStream;

fn main() {
    for i in 0..1024 {
        match TcpStream::connect(format!("scanme.nmap.org:{}", i)){
            Ok(_stream) => println!("{} open", i),
            Err(_err) => ()
        }

    }
}
