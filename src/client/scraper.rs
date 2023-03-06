use dirs::config_dir;
use futures::future::select;
use futures::{channel::mpsc::unbounded, pin_mut};
use futures::{SinkExt, StreamExt};
use lib::protocol::{read_message, ProtocolMessageHeader};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::fs::write_proxies;

// Type alias
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn scraper() -> Result<()> {
    let url = Url::parse("ws://127.0.0.1:54345")?;

    let (_stdin_tx, stdin_rx) = unbounded::<Message>();

    let (mut ws_stream, _) = connect_async(url).await?;
    println!("Handshake completed");

    ws_stream
        .send(Message::Text("REQUEST_PROXIES ".to_string()))
        .await?;

    let (outgoing, incoming) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(outgoing);
    let ws_to_stdout = incoming.for_each(|possible_msg| async {
        if let Ok(msg) = possible_msg {
            if let Ok((msg_header, body)) = read_message(&msg) {
                match msg_header {
                    ProtocolMessageHeader::Proxies => {
                        // println!("{}", body);
                        write_proxies(body).expect("Failed to write proxies to file");
                        println!("Proxies written to {}", config_dir().unwrap().join("proxyster").join("proxies.txt").to_string_lossy());
                    }
                    _ => {}
                }
            }
        };
    });

    pin_mut!(stdin_to_ws, ws_to_stdout);
    select(stdin_to_ws, ws_to_stdout).await;

    Ok(())
}
