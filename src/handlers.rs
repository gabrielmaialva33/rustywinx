use teloxide::prelude::*;

use crate::commands::{Command, ping};

pub async fn handlers(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Ping => {
            let response = ping().await;
            bot.send_message(msg.chat.id, response)
                .await
                .unwrap();
        }
        Command::Start => {
            bot.send_message(msg.chat.id, "Oie :)")
                .send()
                .await
                .unwrap();
        }
    }

    Ok(())
}
