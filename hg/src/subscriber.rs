use tokio::net::TcpStream;
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use std::any::TypeId;
// use std::error::Error;
use crate::messages::Msg;
use std::io;


use std::io::Write;

// TODO: Abstract away TCP to enable other protocols to be used (e.g. UDP)
// trait Stream{
//     fn connect(&self, addr: str);
// }


async fn read_data(stream: &TcpStream, buf: &mut [u8]) -> usize{
    stream.readable().await;
    loop{
        match stream.try_read(buf) {
            Ok(0) => {
                println!("Breaking. TODO: Investigate why it might return 0.");
                // return None;
                return 0;
            }
            Ok(n) => {
                println!("Read {} bytes", n);
                return n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                panic!("ERROR {}", e);
            }
        }
    }
}


pub struct Subscriber<U>{
    pub topic: String,
    pub queue_len: u32, // TODO: Handle queuing on send

    // pub master_addr: String,
    // pub publisher_addr: String,
    // pub listener: TcpListener,
    pub guid: String,

    pub topic_type: String,

    pub callback: fn(msg: U),
}

impl<U: Msg + 'static> Subscriber<U>{
    pub fn new(topic: &str, queue_len: u32, callback: fn(msg: U)) ->  Subscriber<U>{
        
        // TODO: Register with master

        Self{topic: topic.to_owned(),
            queue_len: queue_len.to_owned(),
            // publisher_addr: "".to_owned(),
            // listener: listener,
            guid: String::from(""),
            topic_type: U::dtype(),
            callback: callback}
    }

    pub async fn subscribe(&mut self){
        // let listener = self.listener.clone();
        let callback = self.callback;
        tokio::spawn(async move {
            let addr = String::from("127.0.0.1:8080");
            // let addr = String::from("8.tcp.ngrok.io:13585"); // Working!
            let mut listener = TcpListener::bind(addr.clone()).await.unwrap();
            loop {
                // Asynchronously wait for an inbound socket. This is a new publisher to this topic
                let (mut socket, _) = listener.accept().await.unwrap();
                println!("New connection");
        
                tokio::spawn(async move {
                    let mut buf = vec![0; 1024];
                    match read_data(&socket, &mut buf).await {
                        0 => {
                            println!("0 bytes read. Investigate this.");
                        },
                        _ =>{
                            let msg = U::deserialize(&buf);
                            (callback)(msg);
                        },
                    };
                });
            }
        });
    }

    // pub async fn subscribe(&mut self) {
    //     let stream = &self.master;
    //     tokio::spawn(async move {
    //         loop{
    //             stream.readable().await.unwrap();
    //             let mut buf = [0; 4096];
    //             loop{
    //                 match stream.try_read(&mut buf) {
    //                     Ok(0) => {
    //                         println!("Breaking");
    //                         break;
    //                     }
    //                     Ok(n) => {
    //                         println!("read {:?}", buf);
    //                         break;
    //                     }
    //                     Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    //                         continue;
    //                     }
    //                     Err(e) => {
    //                         // return Err(e.into());
    //                         panic!("ERROR {}", e);
    //                     }
    //                 }
    //             }
    //         }
    //     });
    // }
}

