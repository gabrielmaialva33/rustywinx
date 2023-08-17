extern crate dotenv;

use dotenv::dotenv;
use log::info;
use std::env;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    match dotenv() {
        Ok(_) => info!("Loaded dotenv"),
        Err(_) => info!("Failed to load dotenv"),
    }

    pretty_env_logger::init_timed();

    info!("Starting bot...");

    let bot = Bot::new(env::var("BOT_TOKEN").unwrap());
    bot.set_my_commands(Command::bot_commands()).await.unwrap();

    let me = bot.get_me().await.unwrap().mention();
    info!("... {} started!", me);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_message(msg.chat.id, "Hello, World!")
            .send()
            .await?;
        Ok(())
    })
    .await
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
