use lib::protocol::readMessage;
use ws::{listen, Message};

fn main() {
    if let Err(err) = listen("127.0.0.1:54345", |out| {
        move |msg: Message| {
            println!("Received message: {}", msg);
            readMessage(msg.as_text().unwrap().to_string());
            Ok(())
        }
    }) {
        println!("Failed to create WebSocket due to {:?}", err);
    }
}