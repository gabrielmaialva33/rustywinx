extern crate dotenv;

use dotenv::dotenv;
use lib::bot;
use log::info;

#[tokio::main]
async fn main() {
    match dotenv() {
        Ok(_) => info!("Loaded dotenv"),
        Err(_) => info!("Failed to load dotenv"),
    }

    pretty_env_logger::init_timed();

    bot::run_bot().await.unwrap();
}
