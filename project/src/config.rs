use serenity::model::id::{ChannelId, GuildId};

#[derive(Debug)]
pub struct Config {
    /// Choose the log level of the app: trace, debug, info, warn, error.
    pub log_level: String,
    /// Use the token from your discord application.
    pub discord_token: String,
    /// Guild's id from discord.
    pub guild_id: GuildId,
    /// Channel to post the texts.
    pub channel_id: ChannelId,
    /// The App/bot id of the bot.
    pub bot_id: u64,
    /// After how many seconds you can send a message.
    pub spam_period: i64,
}

impl Config {
    pub fn load_env() -> Config {
        // Get log level
        let log_level = std::env::var("RUST_LOG").expect("Missing RUST_LOG");

        // Get discord token from env.
        let discord_token = std::env::var("DISCORD_TOKEN").expect("'DISCORD_TOKEN' was not found");

        // Get bot_id from env.
        let bot_id: u64 = std::env::var("BOT_ID")
            .expect("'BOT_ID' was not found")
            .parse::<u64>()
            .expect("'BOT_ID' wrong format");

        // Get guild_id from env.
        let guild_id: GuildId = GuildId(
            std::env::var("GUILD_ID")
                .expect("'GUILD_ID' was not found")
                .parse::<u64>()
                .expect("'GUILD_ID' wrong format"),
        );

        // Get channel_id from env.
        let channel_id: ChannelId = ChannelId(
            std::env::var("CHANNEL_ID")
                .expect("'CHANNEL_ID' was not found")
                .parse::<u64>()
                .expect("'CHANNEL_ID' wrong format"),
        );

        // Get spam_period
        let spam_period = std::env::var("SPAM_PERIOD")
            .expect("Missing SPAM_PERIOD")
            .parse::<i64>()
            .expect("'SPAM_PERIOD' wrong format");

        Config {
            log_level,
            discord_token,
            guild_id,
            bot_id,
            channel_id,
            spam_period,
        }
    }
}
