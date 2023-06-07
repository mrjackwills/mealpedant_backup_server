use base64::{engine, Engine};
use futures_util::lock::Mutex;
use futures_util::SinkExt;
use std::sync::Arc;
use tokio::fs;
use tracing::{error, trace};

use crate::app_error::AppError;
use crate::ws_messages::{BackupData, MessageValues, ParsedMessage, Response, StructuredResponse};
use crate::{env::AppEnv, ws_messages::to_struct};

use super::WSWriter;

#[derive(Debug, Clone)]
pub struct WSSender {
    app_envs: AppEnv,
    writer: Arc<Mutex<WSWriter>>,
}

impl WSSender {
    pub fn new(app_envs: &AppEnv, writer: Arc<Mutex<WSWriter>>) -> Self {
        Self {
            app_envs: app_envs.clone(),
            writer,
        }
    }

    /// Handle text message, in this program they will all be json text
    pub async fn on_text(&mut self, message: String) {
        if let Some(data) = to_struct(&message) {
            match data {
                MessageValues::Invalid(error) => error!("{error:?}"),
                MessageValues::Valid(message, unique) => match message {
                    ParsedMessage::Backup => {
                        // Log errors, else just ignore
                        match Self::send_backup(self, unique).await {
                            Ok(_) => trace!("backup sent"),
                            Err(e) => {
                                error!("send_backup::{e}");
                            }
                        }
                    }
                },
            }
        }
    }

    /// Look in backups folder, sort by name (so will sort by data), and send the last file, e.g. the newest, as long as it's a tar.age file
    async fn send_backup(&mut self, unique: String) -> Result<(), AppError> {
        let mut all_files: Vec<String> = vec![];
        let mut entry = fs::read_dir(&self.app_envs.location_backup).await?;
        while let Some(file) = entry.next_entry().await? {
            if let Ok(file_name) = file.file_name().into_string() {
                if !file_name.starts_with('.')
                    && file_name.ends_with(".tar.age")
                    && !file_name.contains("PHOTOS")
                {
                    all_files.push(file_name);
                }
            };
        }
        all_files.sort();

        if let Some(file_name) = all_files.last() {
            let file_to_send = format!("{}/{file_name}", &self.app_envs.location_backup);
            let data_to_send = BackupData {
                file_as_b64: engine::general_purpose::STANDARD
                    .encode(fs::read(file_to_send).await?),
                file_name: file_name.clone(),
            };
            let response = Response::BackupData(data_to_send);
            self.send_message(response, unique).await;
        }

        Ok(())
    }

    /// Send a message to the socket
    pub async fn send_message(&mut self, response: Response, unique: String) {
        match self
            .writer
            .lock()
            .await
            .send(StructuredResponse::data(response, unique))
            .await
        {
            Ok(_) => trace!("Message sent"),
            Err(e) => error!("send_ws_response::SEND-ERROR::{e}"),
        }
    }

    /// close connection, uses a 2 second timeout
    pub async fn close(&mut self) {
        tokio::time::timeout(
            std::time::Duration::from_secs(2),
            self.writer.lock().await.close(),
        )
        .await
        .ok();
    }
}
