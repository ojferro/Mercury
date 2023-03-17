use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::any::TypeId;


use std::io::Write;

// TODO: Abstract away TCP to enable other protocols to be used (e.g. UDP)
// trait Stream{
//     fn connect(&self, addr: str);
// }

pub struct Publisher{
    pub topic: String,
    pub queue_len: u32, // TODO: Handle queuing on send

    pub master_addr: String,
    pub master: TcpStream,
    pub guid: String,

    pub topic_type: TypeId,
}

impl Publisher{
    pub async fn new<T: 'static>(topic: String, queue_len: u32) ->  Publisher{
        let addr = String::from("127.0.0.1:8080");
        let mut stream = TcpStream::connect(addr.clone()).await.unwrap();
        println!("Created the stream in new");
        Self{topic: topic, queue_len: queue_len, master_addr: addr.clone(), master: stream, guid: String::from(""), topic_type: TypeId::of::<T>()}
    }
    pub async fn advertise(&mut self) {
        let msg = format!("Publisher.Topic:{}.Type:{:?}", &self.topic, self.topic_type);
        self.master.write(msg.as_bytes()).await;
        println!("Done, from advertise");
    }
}

