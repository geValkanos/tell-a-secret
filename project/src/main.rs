use serenity::prelude::*;

use log::error;

mod common;
mod config;
mod discord;

use crate::config::Config;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = Config::load_env();

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;

    let mut client = Client::builder(&config.discord_token, intents)
        .event_handler(discord::Bot::new(config))
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
