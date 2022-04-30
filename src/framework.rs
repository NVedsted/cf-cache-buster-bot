use std::collections::HashSet;
use log::{error, info};

use serenity::framework::standard::{Args, CommandGroup, CommandOptions, CommandResult, DispatchError, help_commands, HelpOptions, Reason};
use serenity::framework::standard::macros::{check, help, hook};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::prelude::UserId;
use serenity::prelude::Context;

use crate::command::GENERAL_GROUP;
use crate::config::BotConfigKey;

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[check]
#[name(only_approved_roles)]
async fn only_approved_roles(ctx: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let bot_config = data.get::<BotConfigKey>().unwrap();
    let member = msg.member.as_ref().unwrap();
    if !bot_config.allowed_role_ids.iter()
        .any(|r| member.roles.contains(r)) {
        return Err(Reason::User("You do not have the necessary roles to execute this command.".to_owned()));
    }

    Ok(())
}

#[check]
#[name(only_approved_guilds)]
async fn only_approved_guilds(ctx: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let bot_config = data.get::<BotConfigKey>().unwrap();
    if !bot_config.allowed_guild_ids.contains(msg.guild_id.as_ref().unwrap()) {
        return Err(Reason::User("This command is not allowed to be run in this server.".to_owned()));
    }

    Ok(())
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _command_name: &str) {
    let message = match error {
        DispatchError::CheckFailed(_, Reason::User(m) | Reason::UserAndLog { user: m, .. }) => m,
        DispatchError::CheckFailed(_, _) => "You failed a check.".to_owned(),
        DispatchError::Ratelimited(_) => "You are being rate-limited.".to_owned(),
        DispatchError::CommandDisabled => "This command is disabled.".to_owned(),
        DispatchError::BlockedUser => "You are blocked from this command.".to_owned(),
        DispatchError::BlockedGuild => "This server is blocked from this command.".to_owned(),
        DispatchError::BlockedChannel => "This channel is blocked from this command.".to_owned(),
        DispatchError::OnlyForDM => "This command can only be used in DMs.".to_owned(),
        DispatchError::OnlyForGuilds => "This command can only be used in servers.".to_owned(),
        DispatchError::OnlyForOwners => "This command can only be used by owners.".to_owned(),
        DispatchError::LackingRole => "You do not have the requires role(s).".to_owned(),
        DispatchError::LackingPermissions(_) => "You do not have the requires permission(s).".to_owned(),
        DispatchError::NotEnoughArguments { min, given } => format!("At least {} arguments must be provided. (You provided {})", min, given),
        DispatchError::TooManyArguments { given, max } => format!("At most {} arguments may be provided. (You provided {})", max, given),
        _ => "Unknown error.".to_owned(),
    };

    if let Err(why) = msg.channel_id.say(ctx, format!("Command failed with reason: {}", message)).await {
        error!("Failed to send error message: {:?}", why);
    }
}

#[hook]
async fn after(ctx: &Context, msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => info!("Processed command '{}'", command_name),
        Err(why) => {
            error!("Command '{}' returned error {:?}", command_name, why);
            if let Err(why) = msg.channel_id.say(ctx, "Command failed. Check console output for more information.").await {
                error!("Failed to inform user of error. Reason: {:?}", why);
            }
        }
    }
}

pub fn build_framework(prefix: &str) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| c.prefix(prefix))
        .group(&GENERAL_GROUP)
        .help(&MY_HELP)
        .after(after)
        .on_dispatch_error(dispatch_error)
}
