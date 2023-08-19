use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // this is the bot token
    pub bot_token: String,

    // this is the database path
    pub database_path: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            bot_token: env::var("BOT_TOKEN").unwrap(),
            database_path: env::var("DATABASE_PATH").unwrap(),
        }
    }
}
