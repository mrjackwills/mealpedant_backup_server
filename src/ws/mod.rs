mod connect;
mod connection_details;
mod ws_sender;

use futures_util::lock::Mutex;
use futures_util::{
    StreamExt, TryStreamExt,
    stream::{SplitSink, SplitStream},
};
use std::sync::Arc;
use tokio::{net::TcpStream, task::JoinHandle};
use tokio_tungstenite::{self, MaybeTlsStream, WebSocketStream, tungstenite::Message};

use connect::ws_upgrade;
use connection_details::ConnectionDetails;

use crate::app_env::AppEnv;

use self::ws_sender::WSSender;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WSReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type WSWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

#[derive(Debug, Default)]
struct AutoClose(Option<JoinHandle<()>>);

impl AutoClose {
    /// Will close the connection after 40 seconds, unless it is called within that 40 seconds
    /// Get called on every ping received (server sends a ping every 30 seconds)
    fn on_ping(&mut self, ws_sender: &WSSender) {
        if let Some(handle) = self.0.as_ref() {
            handle.abort();
        }
        let ws_sender = ws_sender.clone();
        self.0 = Some(tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(40)).await;
            ws_sender.close().await;
        }));
    }
}

/// Handle each incoming ws message
async fn incoming_ws_message(mut reader: WSReader, ws_sender: WSSender) {
    let mut auto_close = AutoClose::default();
    auto_close.on_ping(&ws_sender);
    while let Ok(Some(message)) = reader.try_next().await {
        match message {
            Message::Text(message) => {
                let ws_sender = ws_sender.clone();
                tokio::spawn(async move {
                    ws_sender.on_text(message.to_string()).await;
                });
            }
            Message::Ping(_) => auto_close.on_ping(&ws_sender),
            Message::Close(_) => {
                ws_sender.close().await;
                break;
            }
            _ => (),
        }
    }
    tracing::info!("incoming_ws_message done");
}

/// try to open WS connection, and spawn a ThreadChannel message handler
pub async fn open_connection(app_envs: AppEnv) {
    let mut connection_details = ConnectionDetails::new();
    loop {
        tracing::info!("in connection loop, awaiting delay then try to connect");
        connection_details.reconnect_delay().await;
        match ws_upgrade(&app_envs).await {
            Ok(socket) => {
                tracing::info!("connected in ws_upgrade match");
                connection_details.valid_connect();
                let (writer, reader) = socket.split();
                let ws_sender = WSSender::new(&app_envs, Arc::new(Mutex::new(writer)));
                incoming_ws_message(reader, ws_sender).await;
                tracing::info!("aborted spawns, incoming_ws_message done, reconnect next");
            }
            Err(e) => {
                tracing::error!("connect::{e}");
                connection_details.fail_connect();
            }
        }
    }
}
