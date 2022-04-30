use log::info;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::config::{BotConfigKey, load_config};
use crate::framework::build_framework;

mod framework;
mod cloudflare;
mod config;
mod command;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Connected to Discord as {}", ready.user.name);
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> serenity::Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("cf_cache_buster_bot=info")
    ).init();

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let config = load_config().await?;
    let framework = build_framework(&config.command_prefix);
    info!("Initializing framework with '{}' as command prefix.", &config.command_prefix);
    let mut client = Client::builder(&config.bot_token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await?;
    {
        let mut data = client.data.write().await;
        data.insert::<BotConfigKey>(config);
    }

    client.start().await
}
