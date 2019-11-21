# tg-rs-upvote-bot

A (prototype) telegram bot to upvote and query top messages from group chats.

## Setup
Never used Rust, MongoDB or created a Telegram Bot? Read the [quickstart guide](https://github.com/tchx84/tg-rs-upvote-bot/blob/master/quickstart.md).

## Usage

Any user can upvote by replying messages with the `TG_BOT_TAG`, e.g. 🥭, and check most popular messages with the `/top` command, e.g, `/top 10 7days`.

## Debug

```
RUST_LOG=tg_rs_upvote_bot cargo run
```
