use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    pin_mut,
    stream::StreamExt,
    TryStreamExt,
};
use lib::{
    fetch::fetch,
    protocol::{read_message, ProtocolMessageHeader},
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::cli::cli;

mod cli;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn handle_connection(
    peer_map: PeerMap,
    raw_stream: TcpStream,
    addr: SocketAddr,
    proxies: Vec<String>,
) {
    let ws_stream = accept_async(raw_stream)
        .await
        .expect("should be able to accept TCP stream");

    println!(
        "New WS connection: {}",
        ws_stream.get_ref().peer_addr().unwrap()
    );

    let (tx, rx) = unbounded::<Message>();
    peer_map
        .lock()
        .expect("should be able to block the thread and aquire the mutex")
        .insert(addr, tx);

    // split the websocket stream into a sender and receiver
    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg: Message| {
        let mut msg_to_send: Message = Message::Text(String::from(""));

        if let Ok((msg_header, _body)) = read_message(&msg) {
            match msg_header {
                ProtocolMessageHeader::RequesProxies => {
                    let proxies_as_string = proxies.join("\n");
                    msg_to_send = Message::Text(format!("PROXIES {}", proxies_as_string));
                }
                _ => {}
            }
        }

        let peers = peer_map.lock().unwrap();

        let broadcast_recipients = peers
            .iter();
            // .filter(|(peer_addr, _)| peer_addr != &&addr)

        for (sock, recp) in broadcast_recipients {
            println!("Sending to {}:{}", sock.ip(), sock.port());
            recp.unbounded_send(msg_to_send.clone()).unwrap();
        }

        futures::future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);

    futures::future::select(broadcast_incoming, receive_from_others).await;
    

    println!("{} disconnected", addr);
    peer_map.lock().unwrap().remove(&addr);
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _matches = cli();

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:54345").await?;

    // let proxies = thread::spawn(|| async { fetch().expect("TODO: HANDLE THIS ERROR") }).join().unwrap().await;
    let proxies = fetch().await.unwrap();

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(
            state.clone(),
            stream,
            addr,
            proxies.clone(),
        ))
        .await?;
    }

    Ok(())
}
