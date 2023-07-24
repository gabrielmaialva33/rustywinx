extern crate dotenv;

use teloxide::prelude::*;
use log::info;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();

    info!("Starting bot...");

    let bot = Bot::new(env::var("BOT_TOKEN").unwrap());

    teloxide::repl(bot, |bot: Bot, message: Message| async move {
        bot.send_dice(message.chat.id).await?;
        Ok(())
    }).await;
}
