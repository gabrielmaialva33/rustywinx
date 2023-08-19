use anyhow::Error;
use teloxide::Bot;
use teloxide::prelude::Dispatcher;
use teloxide::types::Message;

use crate::modules::ModuleManager;
use crate::types::TeloxideDispatcher;

async fn logging_middleware(bot: Bot, message: Message) -> Message {
    info!("Got a message from {}: {}", message.from.first_name, message.text.unwrap_or_default());
    message
}

pub(crate) async fn build_dispatcher(bot: Bot, mut module_mgr: ModuleManager) -> Result<TeloxideDispatcher, Error> {
    let mut dp = Dispatcher::new(bot.clone())
        .messages_handler(move |rx: DispatcherHandlerRx<Message>| {
            rx.for_each_concurrent(None, |(bot, msg)| async move {
                let msg = logging_middleware(bot, msg).await;
                module_mgr.filter_handler().call(bot, msg).await.unwrap();
            })
        });

    for module in module_mgr.modules.iter_mut() {
        module.register_dependency(&mut dp.dependencies).await?;
        for command in module.commands() {
            dp.add_handler(command.handler);
        }
    }

    Ok(dp)
}