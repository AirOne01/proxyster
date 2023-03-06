use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    pin_mut,
    stream::StreamExt,
    SinkExt, TryStreamExt,
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
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

use crate::cli::cli;

mod cli;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    let mut ws_stream = accept_async(raw_stream)
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
        if let Ok((msg2, _body)) = read_message(&msg) {
            match msg2 {
                ProtocolMessageHeader::RequesProxies => {
                    async fn hopefully_future(ws_stream: &mut WebSocketStream<TcpStream>) {
                        ws_stream.send(Message::Text("REQUEST_PROXIES ".to_string()));
                    }
                    tokio::spawn(hopefully_future(&mut ws_stream));
                    // tokio::spawn(send_proxies(ws_stream));
                }
                _ => {}
            }
        }

        let peers = peer_map.lock().unwrap();

        let broadcast_recipients = peers
            .iter()
            .filter(|(peer_addr, _)| peer_addr != &&addr)
            .map(|(_, ws_sink)| ws_sink);

        for recp in broadcast_recipients {
            println!("sending to {:?}", recp);
            recp.unbounded_send(msg.clone()).unwrap();
        }

        futures::future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);

    futures::future::select(broadcast_incoming, receive_from_others).await;

    // tokio::spawn(|| -> impl std::future::Future<Output = ()> {
    //     futures::future::select(broadcast_incoming, receive_from_others).await;
    // });

    println!("{} disconnected", addr);
    peer_map.lock().unwrap().remove(&addr);
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _matches = cli();

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:54345").await?;

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr)).await?;
    }

    Ok(())
}

async fn send_proxies(ws_stream: WebSocketStream<TcpStream>) -> Result<(), std::io::Error> {
    let mut stream = ws_stream;
    let msg = Message::Text(String::from("PROCESSING "));
    stream
        .send(msg)
        .await
        .expect("should be able to send message");
    let res = fetch().expect("TODO: HANDLE THIS ERROR");
    let msg_to_send = Message::Text(res.join("\n"));
    stream
        .send(msg_to_send)
        .await
        .expect("should be able to send message");
    Ok(())
}
