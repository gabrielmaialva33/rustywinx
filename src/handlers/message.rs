use std::error::Error;

use crate::commands::start::start;
use crate::Command;
use once_cell::sync::OnceCell;
use teloxide::{prelude::*, types::Me, utils::command::BotCommands};

static ME: OnceCell<Me> = OnceCell::new();

pub async fn message_handler(
    bot: Bot,
    message: Message,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = message.text() {
        let _from = message.from().cloned();
        let from = _from.as_ref();

        if from.is_none() {
            return Ok(());
        }

        let mut parsed_command = BotCommands::parse(text, ME.get().unwrap().username());
        if parsed_command.is_err() {
            let splits: Vec<_> = text.splitn(2, ' ').map(|x| x.to_lowercase()).collect();
            let first_word = splits.get(0).map(|x| x.as_str());

            parsed_command = match first_word {
                Some("/start") => Ok(Command::Start),
                _ => parsed_command,
            }
        }

        return match parsed_command {
            Ok(Command::Start) => {
                start(bot, message.chat.id).await?;
                Ok(())
            }

            Err(_) => Ok(()),
        };
    }

    Ok(())
}
