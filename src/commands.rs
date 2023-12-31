use std::time::Instant;

use sys_info::{cpu_num, mem_info, os_type};
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "List of commands:")]
pub enum Command {
    #[command(description = "Ping the bot.")]
    Ping,
    #[command(description = "Start the bot.")]
    Start,
}

pub async fn ping() -> String {
    let start_time = Instant::now();

    info!("start_time: {:?}", start_time);

    let os = os_type().unwrap_or_else(|_| "Unknown OS".to_string());

    let cpu_count = cpu_num().unwrap_or(0);
    let memory = mem_info().unwrap();

    let end_time = Instant::now();

    info!("end_time: {:?}", end_time);
    let duration = end_time.duration_since(start_time);
    let duration_in_milliseconds = duration.as_micros() as f64 / 1000.0;
    info!("duration: {:?}", duration);

    info!("duration.as_millis(): {:?}", duration.as_millis());


    format!(
        "Pong!\n\nResponse Time: {:.2} ms\nOS: {}\nCPU Cores: {}\nTotal Memory: {} MB",
        duration_in_milliseconds,
        os,
        cpu_count,
        memory.total / 1024 // Convert to MB
    )
}

pub async fn start() -> String {
    "Welcome to the bot!".to_string()
}
