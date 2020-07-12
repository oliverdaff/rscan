use futures::{stream, StreamExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    stream::iter(1i32..1024)
        .map(|p| async move {
            TcpStream::connect(format!("scanme.nmap.org:{}", p))
                .await
                .map(|x| (x, p))
        })
        .buffer_unordered(100)
        .for_each(|c| async {
            match c {
                Ok((_, p)) => println!("{} open", p),
                Err(_err) => (),
            }
        })
        .await;
}
