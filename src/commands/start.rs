use std::error::Error;
use teloxide::{prelude::*, types::ParseMode};

pub async fn start(bot: Bot, chat_id: ChatId) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(chat_id, "Hello!")
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}
