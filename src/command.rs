use log::{error, info};
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::{command, group};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use serenity::utils::Colour;

use crate::{BotConfigKey, cloudflare};
use crate::framework::{ONLY_APPROVED_GUILDS_CHECK, ONLY_APPROVED_ROLES_CHECK};

#[group]
#[commands(clearcache)]
struct General;

#[command]
#[checks(only_approved_roles, only_approved_guilds)]
#[description("Clears the CloudFlare cache for the provided URL.")]
#[num_args(1)]
#[usage("<URL>")]
#[only_in("guild")]
async fn clearcache(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = args.single::<String>()?;

    let lock = ctx.data.read().await;
    let config = lock.get::<BotConfigKey>().unwrap();

    if !url.trim().starts_with(&config.url_prefix) {
        msg.channel_id.send_message(&ctx, |m| m.embed(|e| e
            .title("You cannot purge this file!")
            .description(format!("URLs must begin with `{}`.", &config.url_prefix))
            .colour(Colour::GOLD)
        )).await?;
        return Ok(());
    }

    let response = cloudflare::purge_file_cache(
        &config.cf_service_token,
        &config.zone_identifier,
        url.trim(),
    ).await?;

    if response.success {
        info!("Purged file cache for: {}", url);
        msg.channel_id.send_message(&ctx, |m| m.embed(|e| e
            .title("Successfully purged cache!")
            .description(format!("Cache for `{}` has been purged.", url))
            .colour(Colour::DARK_GREEN)
        )).await?;
    } else {
        error!("Failed to purge file cache for: {}. CF errors: {:?}", url, response.errors);
        msg.channel_id.send_message(&ctx, |m| m.embed(|e| e
            .title("One or more errors occurred while clearing cache!")
            .description("CloudFlare reported the following errors:")
            .colour(Colour::RED)
            .fields(response.errors.iter()
                .map(|e| (format!("Error code {}", e.code), &e.message, false))
            )
        )).await?;
    }

    Ok(())
}
