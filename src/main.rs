#![forbid(unsafe_code)]
#![warn(clippy::unused_async, clippy::unwrap_used, clippy::expect_used)]
// Wanring - These are indeed pedantic
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions, clippy::doc_markdown)]
// Only allow when debugging
// #![allow(unused)]

mod app_error;
mod env;
mod word_art;
mod ws;
mod ws_messages;

use env::AppEnv;
use tracing::Level;
use word_art::Intro;
use ws::open_connection;

fn setup_tracing(app_envs: &AppEnv) {
    let level = if app_envs.trace {
        Level::TRACE
    } else if app_envs.debug {
        Level::DEBUG
    } else {
        Level::INFO
    };
    tracing_subscriber::fmt().with_max_level(level).init();
}

#[tokio::main]
async fn main() {
    let app_envs = AppEnv::get();
    setup_tracing(&app_envs);
    Intro::new(&app_envs).show();
    open_connection(app_envs).await;
}
