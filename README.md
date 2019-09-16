# tg-rs-upvote-bot

A (prototype) telegram bot to upvote and query top messages from group chats.

## Setup

```
git clone https://github.com/tchx84/tg-rs-upvote-bot.git;
cd tg-rs-upvote-bot;

cat > .env << "EOF"
TG_BOT_TOKEN="TOKEN"
TG_BOT_TAG="🥭"
TG_BOT_REPLY="☝"
TG_BOT_DB_NAME="upvotes"
TG_BOT_DB_HOST="localhost"
TG_BOT_DB_PORT=27017
EOF

sudo service mongod start;
cargo run;
```

NOTE: **BotFather** must set  **inline-mode** to `off` and **group-privacy** to `off`.

## Usage

Any user can upvote by replying messages with the `TG_BOT_TAG`, e.g. 🥭, and check most popular messages with the `/top` command, e.g, `/top 10 7days`.
