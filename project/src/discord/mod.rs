use log::{error, info};
use serenity::async_trait;
use serenity::client::EventHandler;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::Context;

use crate::common::spam::Spam;
use crate::config::Config;

pub struct Bot {
    channel_id: ChannelId,
    guild_id: GuildId,
    bot_id: u64,
    spam_period: i64,
    hash_map: Spam,
}

impl Bot {
    pub fn new(config: Config) -> Self {
        Bot {
            bot_id: config.bot_id,
            guild_id: config.guild_id,
            channel_id: config.channel_id,
            hash_map: Spam::new(),
            spam_period: config.spam_period,
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        // Create new thread to channel.
        if msg.guild_id.is_none() {
            // Check spam status.
            let current_timestamp: i64 = msg.timestamp.unix_timestamp();
            let is_spam: bool = match self.hash_map.get(msg.author.id.0) {
                Some(timestamp) => timestamp + self.spam_period > current_timestamp,
                None => false,
            };
            if is_spam {
                info!("User needs to wait in order to resend a message")
            } else {
                // Validate if it is a user from the guild.
                if let Err(e) = self.guild_id.member(&ctx.http, msg.author.id).await {
                    error!("User not on the guild: {:?}", e);
                } else {
                    self.hash_map.insert(msg.author.id.0, current_timestamp);
                    // Post to the channel
                    if let Err(e) = self.channel_id.say(&ctx.http, msg.content).await {
                        error!("Error sending message: {:?}", e);
                    }
                }
            }
        } else if self.bot_id == msg.author.id.0 && msg.message_reference.is_none() {
            info!("{:?}", msg);
            let header = match msg.content.len() <= 10 {
                true => msg.content.to_string(),
                false => {
                    let mut x = String::from(&msg.content[..10]);
                    x.push_str("...");
                    x
                }
            };

            if let Err(e) = self
                .channel_id
                .create_public_thread(
                    &ctx.http,
                    msg.id,
                    |t| t.name(header), // .auto_archive_duration(60)
                )
                .await
            {
                error!("Error creating thread: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}
