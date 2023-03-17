
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;
use std::str;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr).await?;
    println!("mercury_master running on {}", addr);

    // Max number of KB (1024 bytes) that an incoming msg can be
    let max_msg_size_kb = 1;

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;
        println!("New connection");

        tokio::spawn(async move {
            let mut buf = vec![0; 1024*max_msg_size_kb];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf[..])
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }
                println!("Received {} bytes.", n);
                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");

                // println!("{:?}", buf);
            }
        });
    }
}