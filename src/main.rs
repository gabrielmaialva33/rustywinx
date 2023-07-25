mod commands;
mod handlers;

extern crate dotenv;

use dotenv::dotenv;
use log::info;
use std::env;
use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    pretty_env_logger::init_timed();

    info!("Starting bot...");

    let bot = Bot::new(env::var("BOT_TOKEN").unwrap());

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handlers::message::message_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Display this text.")]
    Start,
}
