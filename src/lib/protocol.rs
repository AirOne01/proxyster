// The "protocol" used to request/provide proxies from/to the proxy server.
// TODO: obviously add all options to requests [1]

use tokio_tungstenite::tungstenite::Message;

type ProtocolMessage = (ProtocolMessageHeader, String);

// ProtocolMessageHeader to String
impl ProtocolMessageHeader {
    fn as_str(&self) -> &'static str {
        match self {
            ProtocolMessageHeader::RequesProxies => "REQUEST_PROXIES",
            ProtocolMessageHeader::Goodbye => "GOODBYE",
            ProtocolMessageHeader::Processing => "PROCESSING",
            ProtocolMessageHeader::Done => "DONE",
            ProtocolMessageHeader::Proxies => "PROXIES",
        }
    }
}

// Message to ProtocolMessageHeader
impl From<&str> for ProtocolMessageHeader {
    fn from(s: &str) -> Self {
        match s {
            "REQUEST_PROXIES" => ProtocolMessageHeader::RequesProxies,
            "GOODBYE" => ProtocolMessageHeader::Goodbye,
            "PROCESSING" => ProtocolMessageHeader::Processing,
            "DONE" => ProtocolMessageHeader::Done,
            "PROXIES" => ProtocolMessageHeader::Proxies,
            _ => {
                panic!("Unknown ProtocolMessageHeader, header was this: {}", s)
            },
        }
    }
}

// == for ProtocolMessageHeader
impl PartialEq for ProtocolMessageHeader {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

pub enum ProtocolMessageHeader {
    // ### CLIENT MESSAGES ###
    RequesProxies, // TODO: [1] here
    Goodbye,       // process stopped normally (client received ^C), kthxbye
    // ### SERVER MESSAGES ###
    Processing, // ok, request received, processing request...
    Done,       // done processing request, here are your proxies sir
    Proxies,      // send a proxy to the client
}

// TODO: eventually this will be replaced by a properly optimized protocol
// pub fn sendRequest(out: ws::Sender, prot_message: ProtocolMessage) {
//     let msg = ws::Message::text(format!("{}", prot_message));
//     out.send(msg).unwrap();
// }

pub fn read_message(msg: &Message) -> Result<ProtocolMessage, &'static str> {
    let args = msg.to_text().unwrap().split(" ").collect::<Vec<&str>>();

    match ProtocolMessageHeader::from(args[0]) {
        ProtocolMessageHeader::RequesProxies => {
            Ok((ProtocolMessageHeader::RequesProxies, String::from("")))
        },
        ProtocolMessageHeader::Proxies => {
            Ok((ProtocolMessageHeader::Proxies, String::from(args[1])))
        },
        _ => Err("Unable to decode message (Not yet implemented)"),
    }
}
