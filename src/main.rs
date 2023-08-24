#![forbid(unsafe_code)]
// Warning - These are indeed pedantic
#![warn(
    clippy::expect_used,
    clippy::nursery,
    clippy::pedantic,
    clippy::todo,
    clippy::unused_async,
    clippy::unwrap_used
)]
#![allow(clippy::module_name_repetitions, clippy::doc_markdown)]
// Only allow when debugging
// #![allow(unused)]

mod app_error;
mod app_env;
mod word_art;
mod ws;
mod ws_messages;

use app_env::AppEnv;
use word_art::Intro;
use ws::open_connection;

fn setup_tracing(app_env: &AppEnv) {
    tracing_subscriber::fmt()
        .with_max_level(app_env.log_level)
        .init();
}

#[tokio::main]
async fn main() {
    let app_envs = AppEnv::get();
    setup_tracing(&app_envs);
    Intro::new(&app_envs).show();
    open_connection(app_envs).await;
}
