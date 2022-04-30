# CloudFlare Cache Buster Bot

This bot allows for people with certain roles on certain Discord servers to clear the cache for URLs beginning with a
configured prefix.

## Command

The following command is made available:

```
!clearcache <URL>
```

Where `!` can be configured to be something else. This will purge the CloudFlare cache for the given URL by
communicating with the [Purge Files by URL](https://api.cloudflare.com/#zone-purge-files-by-url) endpoint.

This will fail if the URL does not begin with the configured URL prefix.

## Configuration

All configuration takes place in `config.json`. As a starting point, you can copy `config.json.example`. The fields are:

| Name                | Description                                                         |
|---------------------|---------------------------------------------------------------------|
| `allowed_role_ids`  | An array containing the IDs of roles allowed to purge  cache.       |
| `allowed_guild_ids` | An array containing the IDs of guilds where purging can take place. |
| `cf_service_token`  | A CloudFlare service token with the permission `#cache_purge:edit`. |
| `zone_identifier`   | An identifier for the zone where the purging will take place.       |
| `bot_token`         | A bot token for the Discord bot.                                    |
| `url_prefix`        | A prefix that all purged URLs must begin with.                      |
| `command_prefix`    | The prefix used in front of commands, e.g. '!'.                     |

## Running

To run the bot, ensure `config.json` is present and properly set up, then simply do:

```commandline
cargo run --release
```

## Logging

Logging can be customized with the environment variable `RUST_LOG`. See [`env_logger`](https://docs.rs/env_logger/) for
more info.
