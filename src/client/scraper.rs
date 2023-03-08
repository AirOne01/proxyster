use dirs::config_dir;
use futures::future::select;
use futures::{channel::mpsc::unbounded, pin_mut};
use futures::{SinkExt, StreamExt};
use lib::log::make_logger;
use lib::protocol::{read_message, ProtocolMessageHeader};
use slog::info;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::fs::write_proxies;

#[tokio::main]
pub async fn scraper() -> Result<(), &'static str> {
    let logger = make_logger();

    let url = match Url::parse("ws://127.0.0.1:54345") {
        Ok(url) => url,
        Err(_) => return Err("Could not parse url"),
    };

    let (_stdin_tx, stdin_rx) = unbounded::<Message>();

    let mut ws_stream = match connect_async(url).await {
        Ok((ws_stream, _)) => ws_stream,
        Err(_) => return Err("Could not connect to server. Is it running ?"),
    };
    info!(logger, "Handshake completed");

    match ws_stream
        .send(Message::Text("REQUEST_PROXIES ".to_string()))
        .await
    {
        Ok(_) => {}
        Err(_) => return Err("Could not send message to server"),
    };

    let (outgoing, incoming) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(outgoing);
    let ws_to_stdout = incoming.for_each(|possible_msg| async {
        if let Ok(msg) = possible_msg {
            if let Ok((msg_header, body)) = read_message(&msg) {
                match msg_header {
                    ProtocolMessageHeader::Proxies => {
                        write_proxies(body).expect("Failed to write proxies to file");
                        match config_dir() {
                            Some(config_dir) => {
                                info!(
                                    logger,
                                    "Proxies written to {}",
                                    config_dir
                                        .join("proxyster")
                                        .join("proxies.txt")
                                        .to_string_lossy()
                                );
                            },
                            None => {
                                info!(logger, "Proxies written to proxies.txt. Could not find the output file when sending that message.");
                            }
                        }
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
