use std::collections::HashSet;

use serde::Deserialize;
use serenity::model::prelude::{GuildId, RoleId};
use serenity::prelude::TypeMapKey;

#[derive(Deserialize, Debug)]
pub struct BotConfig {
    pub allowed_role_ids: HashSet<RoleId>,
    pub allowed_guild_ids: HashSet<GuildId>,
    pub cf_service_token: String,
    pub zone_identifier: String,
    pub bot_token: String,
    pub url_prefix: String,
    pub command_prefix: String,
}

pub struct BotConfigKey;

impl TypeMapKey for BotConfigKey {
    type Value = BotConfig;
}

pub async fn load_config() -> serenity::Result<BotConfig> {
    let file_bytes = tokio::fs::read("config.json").await?;
    serde_json::from_slice::<BotConfig>(&file_bytes)
        .map_err(|e| e.into())
}
