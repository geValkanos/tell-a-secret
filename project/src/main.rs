use serenity::prelude::*;

use log::error;

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
        .event_handler(discord::Bot {
            bot_id: config.bot_id,
            guild_id: config.guild_id,
            channel_id: config.channel_id,
            hash_map: discord::Spam {
                ..Default::default()
            },
            spam_period: config.spam_period,
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
