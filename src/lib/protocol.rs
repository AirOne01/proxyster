// The "protocol" used to request/provide proxies from/to the proxy server.
// TODO: obviously add all options to requests [1]

use std::fmt;

use tokio_tungstenite::tungstenite::Message;

pub struct ProtocolMessage {
    header: ProtocolMessageHeader,
    body: String,
}

// ProtocolMessage to String
impl fmt::Display for ProtocolMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.header, self.body)
    }
}

// ProtocolMessageHeader to String
impl fmt::Display for ProtocolMessageHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            ProtocolMessageHeader::RequesProxies => "REQUEST_PROXIES",
            ProtocolMessageHeader::Goodbye => "GOODBYE",
            ProtocolMessageHeader::Processing => "PROCESSING",
            ProtocolMessageHeader::Done => "DONE",
            ProtocolMessageHeader::Proxy => "PROXY",
        })
    }
}

pub enum ProtocolMessageHeader {
    // ### CLIENT MESSAGES ###
    RequesProxies, // TODO: [1] here
    Goodbye, // process stopped normally (client received ^C), kthxbye
    // ### SERVER MESSAGES ###
    Processing, // ok, request received, processing request...
    Done, // done processing request, here are your proxies sir
    Proxy, // send a proxy to the client
}

// TODO: eventually this will be replaced by a properly optimized protocol
// pub fn sendRequest(out: ws::Sender, prot_message: ProtocolMessage) {
//     let msg = ws::Message::text(format!("{}", prot_message));
//     out.send(msg).unwrap();
// }

pub fn read_message(msg: Message) -> Result<ProtocolMessage, &'static str> {
    if msg.to_string() == format!("{} ", ProtocolMessageHeader::RequesProxies) {
        println!("Received correct request for proxies");
        return Ok(ProtocolMessage {
            header: ProtocolMessageHeader::RequesProxies,
            body: String::new(),
        });
    }
    Err("Unable to decode message (Not yet implemented)")
}