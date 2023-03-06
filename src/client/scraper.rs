use futures::future::select;
use futures::{channel::mpsc::unbounded, pin_mut};
use futures::{SinkExt, StreamExt};
use lib::protocol::{read_message, ProtocolMessageHeader};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

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
    let ws_to_stdout = incoming.for_each(|msg| async {
        if let Ok(data) = msg {
            if let Ok((msg, body)) = read_message(&data) {
                if msg == ProtocolMessageHeader::Proxy {
                    println!("Received message: {}", body);
                };
            }

            tokio::io::stdout()
                .write_all(&(data.into_data()))
                .await
                .unwrap();
        };
    });

    pin_mut!(stdin_to_ws, ws_to_stdout);
    select(stdin_to_ws, ws_to_stdout).await;

    Ok(())
}
