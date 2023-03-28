use serenity::prelude::*;
use shuttle_secrets::SecretStore;

mod common;
mod config;
mod discord;

use crate::config::Config;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let config = Config::load_env(&secret_store);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;

    let client = Client::builder(&config.discord_token, intents)
        .event_handler(discord::Bot::new(config))
        .await
        .expect("Err creating client");

    Ok(client.into())
}
