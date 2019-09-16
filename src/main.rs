use chrono::Utc;
use dotenv::dotenv;
use env_logger;
use futures::Future;
use log;
use mongodb::coll::options::{FindOptions, UpdateOptions};
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson};
use mongodb::{Client, ThreadedClient};
use regex::Regex;
use serde::Deserialize;
use std::env;
use tgbot::{
    handle_updates,
    methods::SendMessage,
    types::{Message, MessageKind, Update, UpdateKind},
    Api, Config, UpdateHandler, UpdateMethod,
};
use time::Duration;

macro_rules! clamp {
    ($value:expr, $min:expr, $max:expr) => {
        if $value > $max {
            $max
        } else if $value < $min {
            $min
        } else {
            $value
        }
    };
}

#[derive(Deserialize, Debug)]
struct Candidate {
    message_id: i64,
    user_id: i64,
    group_id: i64,
}

struct Handler {
    api: Api,
    tag: String,
    reply: String,
    storage: Storage,
}

impl Handler {
    fn handle_commands(&self, message: &Message) {
        let text = match message.get_text() {
            Some(text) => text,
            None => return,
        };

        let chat_id = message.get_chat_id();
        let regex = Regex::new(r"^(/top) +(\d+) +(\d+)").unwrap();
        let captures = match regex.captures(&text.data) {
            Some(captures) => captures,
            None => return,
        };

        let top = clamp!(
            captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            1,
            5
        );
        let days = clamp!(
            captures.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            1,
            365
        );
        log::debug!("/top {} {}", top, days);

        let candidates: Vec<Candidate> = self.storage.find(top, days);
        for candidate in candidates.iter() {
            let method =
                SendMessage::new(chat_id, &self.reply).reply_to_message_id(candidate.message_id);
            self.api
                .spawn(self.api.execute(method).then(|_| Ok::<(), ()>(())));
        }
    }

    fn handle_candidates(&self, message: &Message) {
        let text = match message.get_text() {
            Some(text) => text,
            None => return,
        };

        if !text.data.starts_with(&self.tag) {
            return;
        }

        let replied = match &message.reply_to {
            Some(replied) => replied,
            None => return,
        };

        let (group, user) = match &replied.kind {
            MessageKind::Group { chat, from } => (chat, from),
            _ => return,
        };

        log::debug!("vote for {} {} {}", replied.id, user.id, group.id);
        let candidate = Candidate {
            message_id: replied.id,
            user_id: user.id,
            group_id: group.id,
        };
        self.storage.save(candidate);
    }
}

impl UpdateHandler for Handler {
    fn handle(&mut self, update: Update) {
        let message = match &update.kind {
            UpdateKind::Message(message) => message,
            _ => return,
        };

        self.handle_commands(message);
        self.handle_candidates(message);
    }
}

struct Storage {
    client: Client,
    db: String,
}

impl Storage {
    fn new(db: String, host: String, port: u16) -> Storage {
        Storage {
            client: Client::connect(&host, port).unwrap(),
            db,
        }
    }

    fn save(&self, candidate: Candidate) {
        let collection = self.client.db(&self.db).collection("candidates");
        collection
            .update_one(
                doc! {
                "message_id": candidate.message_id,
                "user_id": candidate.user_id,
                "group_id": candidate.group_id },
                doc! {
                "$inc": doc! {
                    "votes": 1, },
                "$setOnInsert": doc! {
                    "created": Bson::from(Utc::now())}},
                Some(UpdateOptions {
                    upsert: Some(true),
                    write_concern: None,
                }),
            )
            .ok()
            .expect("Failed to save.");
    }

    fn find(&self, top: i64, days: i64) -> Vec<Candidate> {
        let mut candidates: Vec<Candidate> = Vec::new();

        let now = Utc::now();
        let from = now - Duration::days(days);
        let query = doc! {
            "created": doc! {
                "$gte": Bson::from(from),
                "$lt": Bson::from(now) },
        };

        let mut options = FindOptions::new();
        options.sort = Some(doc! { "votes": -1 });
        options.limit = Some(top);

        let collection = self.client.db(&self.db).collection("candidates");
        let cursor = collection.find(Some(query), Some(options)).unwrap();
        for result in cursor {
            if let Ok(item) = result {
                let candidate = bson::from_bson(Bson::Document(item)).unwrap();
                candidates.push(candidate);
            }
        }

        return candidates;
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let tag = env::var("TG_BOT_TAG").expect("TG_BOT_TAG is not set");
    let reply = env::var("TG_BOT_REPLY").expect("TG_BOT_REPLY is not set");
    let token = env::var("TG_BOT_TOKEN").expect("TG_BOT_TOKEN is not set");
    let db = env::var("TG_BOT_DB_NAME").expect("TG_BOT_DB_NAME is not set");
    let host = env::var("TG_BOT_DB_HOST").expect("TG_BOT_DB_HOST is not set");
    let port = env::var("TG_BOT_DB_PORT").expect("TG_BOT_DB_PORT is not set");

    let config = Config::new(token);
    let api = Api::new(config).expect("Failed to create API");
    let storage = Storage::new(db, host, port.parse::<u16>().unwrap());
    tokio::run(handle_updates(
        UpdateMethod::poll(api.clone()),
        Handler {
            api,
            tag,
            reply,
            storage,
        },
    ));
}
