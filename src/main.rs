mod app_env;
mod app_error;
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
    tokio::spawn(open_connection(app_envs)).await.ok();
}
