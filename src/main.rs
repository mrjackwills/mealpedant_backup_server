#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod app_env;
mod app_error;
mod macros;
mod message_handler;
mod word_art;
mod ws;
mod ws_messages;

use app_env::AppEnv;
use async_channel::Sender;
use simple_signal::Signal;
use word_art::Intro;

use crate::{
    app_error::AppError,
    message_handler::{MessageHandler, Msg},
};

fn setup_tracing(app_env: &AppEnv) {
    tracing_subscriber::fmt()
        .with_max_level(app_env.log_level)
        .init();
}

fn close_signal(tx: &Sender<Msg>) {
    let tx = C!(tx);
    simple_signal::set_handler(&[Signal::Int, Signal::Term], move |_| {
        tx.send_blocking(Msg::Exit).ok();
        std::thread::sleep(std::time::Duration::from_millis(250));
        std::process::exit(1);
    });
}

async fn start() -> Result<(), AppError> {
    let app_env = AppEnv::get();
    setup_tracing(&app_env);
    Intro::new(&app_env).show();
    let (tx, rx) = async_channel::bounded(2048);
    close_signal(&tx);
    MessageHandler::new(app_env, rx, tx).start().await;
    Ok(())
}
#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        if let Err(e) = start().await {
            tracing::error!("{e:?}");
        }
    })
    .await
    .ok();
}
