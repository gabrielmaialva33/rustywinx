extern crate dotenv;

use log::info;

use lib::bot;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init_timed();

    bot::run_bot().await.unwrap();

    info!("Bot finished!")
}
