use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::any::TypeId;
use std::io;
use futures::executor::block_on;
use crate::messages::Msg;
use std::io::Write;

// TODO: Abstract away TCP to enable other protocols to be used (e.g. UDP)
// trait Stream{
//     fn connect(&self, addr: str);
// }

pub struct Schema{}
impl Schema{
    pub fn advertise_schema(topic: &str, topic_type: &str, pub_addr: &str) -> String {
        format!("Publisher.Topic:{}.Type:{}.Addr:{}", topic, topic_type, pub_addr)
    }
}

pub struct Publisher{
    pub topic: String,
    pub topic_type: String,

    pub queue_len: u32, // TODO: Handle queuing on send

    // pub master_addr: &str,
    // pub master: TcpStream,
    pub guid: String, // First 4 digits of guid are the port that publisher binds to.

    pub publisher_addr: String,
    pub publisher: TcpStream,
}

async fn read_reply(stream: &TcpStream, buf: &mut [u8]) -> usize{
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

async fn connect_to_master(topic: &str, topic_type: &str, pub_addr: &str) -> String{
    // Connect to master
    let master_address = "127.0.0.1:5923"; // Addr can be ngrok tcp addr, it works!
    let mut master = TcpStream::connect(master_address).await.unwrap();
    
    // Advertise new publisher with master node
    let msg = Schema::advertise_schema(topic, topic_type, pub_addr);
    master.write(msg.as_bytes()).await;
    
    // Await reply and return guid
    let mut buf = [0_u8; 1024];
    let guid = match read_reply(&master, &mut buf).await {
        0 => {
            println!("Returned 0. Investigate this.");
            "BAD_GUID"
        },
        _ =>{
            let s = match std::str::from_utf8(&buf) {
                Ok(s) => s,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            s
        },
    };

    guid.to_owned()
}

impl Publisher{
    pub async fn new<T: 'static + Msg>(topic: &str, queue_len: u32) ->  Publisher{

        let topic_type = T::dtype();
        let pub_addr = "127.0.0.1:8080".to_owned();
        let pub_stream = TcpStream::connect(pub_addr.clone()).await.unwrap();

        let guid = block_on(connect_to_master(topic, &topic_type, &pub_addr));

        Self{topic: topic.to_owned(),
            topic_type: topic_type.to_owned(),
            queue_len: queue_len,
            // master_addr: addr.clone(),
            // master: master,
            guid: guid,
            publisher_addr: pub_addr.clone(),
            publisher: pub_stream,
        }
    }
    pub async fn publish<T: 'static + Msg>(&mut self, msg: &T) {
        // let msg = format!("Publisher.Topic:{}.Type:{:?}", &self.topic, self.topic_type);
        let b = &msg.serialize();
        self.publisher.write(b);
        println!("Publish");
    }
}

