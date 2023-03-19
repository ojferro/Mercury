use tokio::net::TcpStream;
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

pub type Callback = fn();

pub struct Subscriber{
    pub topic: String,
    pub queue_len: u32, // TODO: Handle queuing on send

    // pub master_addr: String,
    pub publisher_addr: String,
    pub guid: String,

    pub topic_type: String,

    pub callback: Callback,
}

impl Subscriber{
    pub async fn new<T: 'static + Msg>(topic: &str, queue_len: u32, callback: Callback) ->  Subscriber{
        let addr = String::from("127.0.0.1:5923");
        // let addr = String::from("8.tcp.ngrok.io:13585"); // Working!
        let mut stream = TcpStream::connect(addr.clone()).await.unwrap();

        tokio::spawn(async move {
            loop{
                stream.readable().await.unwrap();
                let mut buf = [0; 4096];
                loop{
                    match stream.try_read(&mut buf) {
                        Ok(0) => {
                            println!("Breaking");
                            break;
                        }
                        Ok(n) => {
                            println!("read {:?}", buf);
                            callback();
                            break;
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            // return Err(e.into());
                            panic!("ERROR {}", e);
                        }
                    }
                }
            }
        });

        Self{topic: topic.to_owned(),
            queue_len: queue_len.to_owned(),
            publisher_addr: "".to_owned(),
            // master: stream,
            guid: String::from(""),
            topic_type: T::dtype(),
            callback: callback}
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

