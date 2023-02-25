use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    stream::StreamExt, TryStreamExt, pin_mut,
};
use lib::protocol::read_message;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    let ws_stream = accept_async(raw_stream)
        .await
        .expect("Unable to accept TCP stream");

    println!(
        "New WS connection: {}",
        ws_stream.get_ref().peer_addr().unwrap()
    );

    let (tx, rx) = unbounded::<Message>();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {

        read_message(msg.clone()).unwrap();

        let peers = peer_map.lock().unwrap();

        let broadcast_recipients = peers
            .iter()
            .filter(|(peer_addr, _)| peer_addr != &&addr)
            .map(|(_, ws_sink)| ws_sink);

        for recp in broadcast_recipients {
            recp.unbounded_send(msg.clone()).unwrap();
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
async fn main() -> Result<()> {
    let state = PeerMap::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:54345").await?;

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())
}
