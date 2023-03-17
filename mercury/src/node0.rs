use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use std::io;
use std::str;
use std::{thread, time};

use hg::Msg;
use hg::Vec3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Write some data.
    loop{
        let now = time::Instant::now();
        // stream.write_all(b"hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!!hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!!hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!!hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the mesTHIS ARE THE LAST 26 BYTES").await?;
        // stream.write_all(b"hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!!hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!!hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!!hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the message!!! hello world from node0.rs! This is a much longer string now, let's see how it affects the performance and the time it takes to send the mesTHIS ARE THE LAST 26 BYTES").await?;
        // stream.write_all(b"hello").await?;
        let v = hg::Vec3{x:1029824.1,y:1029824.2,z:1029824.3333};
        let b = &v.serialize();
        stream.write_all(b).await?;
        stream.readable().await?;
        let mut buf = [0; 4096];
        

        loop{
            match stream.try_read(&mut buf) {
                Ok(0) => {
                    println!("Breaking");
                    break;
                }
                Ok(n) => {
                    println!("Time elapsed: {:?}", now.elapsed());
                    break;
                    // println!("read {:?}", buf);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        let mut v2 = hg::Vec3{..Default::default()};
        v2.deserialize(&buf);
        println!("x {}, y{}, z{}",v2.x, v2.y, v2.z);

        thread::sleep(time::Duration::from_millis(500));
    }

    Ok(())
}