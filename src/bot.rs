use teloxide::{prelude::*, utils::command::BotCommands};

use crate::handlers::handlers;
use crate::{commands::Command, config::Config};

async fn init_bot(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting bot...");

    let bot = Bot::new(&config.bot_token);
    bot.set_my_commands(Command::bot_commands()).await.unwrap();

    let me = bot.get_me().await.unwrap().mention();

    info!("... {} started!", me);

    Command::repl(bot, handlers).await;

    Ok(())
}

pub async fn run_bot() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();

    init_bot(&config).await?;

    Ok(())
}
