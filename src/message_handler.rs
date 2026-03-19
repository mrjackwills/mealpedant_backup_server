use async_channel::{Receiver, Sender};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::{
    C,
    app_env::AppEnv,
    ws::{ConnectionDetails, Socket, WSSender, open_connection},
    ws_messages::Response,
};

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub type WSReader =
    futures_util::stream::SplitStream<Box<WebSocketStream<MaybeTlsStream<TcpStream>>>>;
pub type WSWriter = futures_util::stream::SplitSink<
    Box<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    tokio_tungstenite::tungstenite::Message,
>;

#[derive(Debug)]
pub struct WSResponse {
    pub response: Response,
    pub unique: String,
}

#[derive(Debug)]
pub enum Msg {
    Exit,
    Ping,
    Received(String),
    ToSend(WSResponse),
    WsClose,
    WsConnected(Box<WsStream>),
}

#[derive(Debug)]
pub struct MessageHandler {
    app_env: AppEnv,
    rx: Receiver<Msg>,
    connection_details: ConnectionDetails,
    socket: Option<Socket>,
    tx: Sender<Msg>,
    ws_sender: WSSender,
}

impl MessageHandler {
    /// Start the message handler
    pub async fn start(&mut self) {
        open_connection(&self.app_env, &self.tx, &mut self.connection_details).await;

        while let Ok(msg) = self.rx.recv().await {
            match msg {
                Msg::Exit => {
                    if let Some(socket) = &mut self.socket {
                        socket.close().await;
                    }
                }
                Msg::Ping => {
                    if let Some(socket) = &mut self.socket {
                        socket.on_ping(&self.tx);
                    }
                }
                Msg::Received(msg) => {
                    let ws_sender = C!(self.ws_sender);
                    tokio::spawn(async move {
                        ws_sender.on_text(msg).await;
                    });
                }
                Msg::ToSend(ws_response) => {
                    if let Some(socket) = &mut self.socket {
                        socket.send(ws_response.response, ws_response.unique).await;
                    }
                }
                Msg::WsClose => {
                    if let Some(socket) = &mut self.socket {
                        socket.close().await;
                    }
                    open_connection(&self.app_env, &self.tx, &mut self.connection_details).await;
                    self.ws_sender.on_connection();
                }
                Msg::WsConnected(stream) => {
                    self.socket = Some(Socket::new(stream, &self.tx));
                }
            }
        }
    }

    pub fn new(app_env: AppEnv, rx: Receiver<Msg>, tx: Sender<Msg>) -> Self {
        let ws_sender = WSSender::new(&app_env, &tx);

        Self {
            app_env,
            connection_details: ConnectionDetails::new(),
            rx,
            socket: None,
            tx,
            ws_sender,
        }
    }
}
