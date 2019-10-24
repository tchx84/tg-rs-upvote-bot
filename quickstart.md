# Quickstart
## Installing Rust
First, you'll have to install Rust. Rust is installed via [rustup](https://rustup.rs/).
The website will automatically provide the right instructions to install Rust.
When you're done, make sure to reload your shell.
To see if Rust is working, run `cargo help`.

## Installing MongoDB
For Windows and macOS, you can get the MongoDB installer [here](https://www.mongodb.com/download-center/community).
If you're on Linux, MongoDB has step-by-step guides for various distros available [here](https://docs.mongodb.com/manual/administration/install-on-linux/).

## Setting up a bot on Telegram
To create a bot, you'll have to chat with the [Botfather](https://t.me/botfather).
To start the conversation, press the start button.
The Botfather then presents you a list of options, to create a new bot, choose `/newbot`.
Now you can give your bot a name and a username.
Now you have to run `/token` to get your token.
That's it, you're ready to start the bot!

## Starting the bot
You can now clone the bot repository and enter it.
```git clone https://github.com/tchx84/tg-rs-upvote-bot.git
cd tg-rs-upvote-bot```

To create the bot config, enter the following command. Make sure to replace TOKEN with your token from the Botfather.
```cat > .env << "EOF"
TG_BOT_TOKEN="TOKEN"
TG_BOT_TAG="🥭"
TG_BOT_REPLY="☝"
TG_BOT_DB_NAME="upvotes"
TG_BOT_DB_HOST="localhost"
TG_BOT_DB_PORT=27017
EOF```

You're ready to go! Just run the following command to run the bot.
```cargo run```