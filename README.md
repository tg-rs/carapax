# TG-RS

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/tg-rs/tg-rs/CI?style=flat-square)](https://github.com/tg-rs/tg-rs/actions/)
[![Codecov](https://img.shields.io/codecov/c/github/tg-rs/tg-rs.svg?style=flat-square)](https://codecov.io/gh/tg-rs/tg-rs)
[![Telegram Chat](https://img.shields.io/badge/telegram-@tgrsusers-blue?style=flat-square)](https://t.me/tgrsusers)

## Project layout

- [tgbot](tgbot) - A Telegram Bot API client
- [carapax](carapax) - A Telegram Bot framework
- [carapax-access](carapax-access) - An access handler
- [carapax-i18n](carapax-i18n) - An i18n utilities
- [carapax-ratelimit](carapax-ratelimit) - A ratelimit handler
- [carapax-session](carapax-session) - A session utilities

## Usage

### tgbot

tgbot is a core library which provides access to Telegram Bot API.

Using longpolling:
```rust no_run
use std::env;
use tgbot::{Api, Config, UpdateHandler, async_trait};
use tgbot::longpoll::LongPoll;
use tgbot::methods::SendMessage;
use tgbot::types::{Update, UpdateKind};

struct Handler {
    api: Api,
}

#[async_trait]
impl UpdateHandler for Handler {
    async fn handle(&mut self, update: Update) {
        println!("got an update: {:?}\n", update);
        if let UpdateKind::Message(message) = update.kind {
            if let Some(text) = message.get_text() {
                let api = self.api.clone();
                let chat_id = message.get_chat_id();
                let method = SendMessage::new(chat_id, text.data.clone());
                api.execute(method).await.unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let api = Api::new(Config::new(token)).expect("Failed to create API");
    LongPoll::new(api.clone(), Handler { api }).run().await;
}
```

Using webhook:
```rust no_run
use tgbot::{types::Update, async_trait, webhook, UpdateHandler};

struct Handler;

#[async_trait]
impl UpdateHandler for Handler {
    async fn handle(&mut self, update: Update) {
        println!("got an update: {:?}\n", update);
    }
}

#[tokio::main]
async fn main() {
    webhook::run_server(([127, 0, 0, 1], 8080), "/", Handler).await.unwrap();
}
```

### carapax

Carapax is a high-level library which provides a more ergonomic way to handle updates:
```rust no_run
use carapax::longpoll::LongPoll;
use carapax::methods::SendMessage;
use carapax::{handler, Api, Command, Config, Dispatcher, ExecuteError};
use std::env;

#[handler(command = "/user_id")]
async fn handle_user_id(api: &Api, command: Command) -> Result<(), ExecuteError> {
    println!("handle /user_id command\n");
    let message = command.get_message();
    let chat_id = message.get_chat_id();
    let method = SendMessage::new(chat_id, format!("Your ID is: {:?}", message.get_user().map(|u| u.id)));
    let result = api.execute(method).await?;
    println!("sendMessage result: {:?}\n", result);
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let api = Api::new(Config::new(token)).expect("Failed to create API");
    let mut dispatcher = Dispatcher::new(api.clone());
    dispatcher.add_handler(handle_user_id);
    LongPoll::new(api, dispatcher).run().await;
}
```

### carapax-access

An access handler

```rust no_run
use carapax::Dispatcher;
use carapax_access::{AccessHandler, AccessRule, InMemoryAccessPolicy};

// Deny from all except for @username (specify without @)
let rule = AccessRule::allow_user("username");
let policy = InMemoryAccessPolicy::default().push_rule(rule);

let mut dispatcher = Dispatcher::new(());
dispatcher.add_handler(AccessHandler::new(policy));
// add other handlers here
```

Note that you should add access handler before any other handlers you want to protect.

### carapax-i18n

Utilites for gettext

```rust no_run
use carapax::{handler, longpoll::LongPoll, methods::SendMessage, types::Update, Api, Config, Dispatcher};
use carapax_i18n::{Catalog, Translator, TranslatorStore};
use std::env;

const RU: &[u8] = include_bytes!("../../carapax-i18n/data/ru.mo");
const EN: &[u8] = include_bytes!("../../carapax-i18n/data/en.mo");

struct Context {
    api: Api,
    translators: TranslatorStore,
}

#[handler]
async fn update_handler(context: &Context, update: Update) {
    let translator = context.translators.get_translator(&update);
    println!("GOT UPDATE: {:?}; LOCALE: {:?}", update, translator.get_locale());
    context
        .api
        .execute(SendMessage::new(
            update.get_chat_id().unwrap(),
            translator.translate("Hello, stranger!"),
        ))
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let api = Api::new(Config::new(token)).expect("Failed to create API");
    let en = Translator::new("en", Catalog::parse(EN).unwrap());
    let ru = Translator::new("ru", Catalog::parse(RU).unwrap());
    let translators = TranslatorStore::new(en).add_translator(ru);
    let mut dispatcher = Dispatcher::new(Context {
        api: api.clone(),
        translators,
    });
    dispatcher.add_handler(update_handler);
    LongPoll::new(api, dispatcher).run().await
}
```

### carapax-ratelimit

Allows to limit number of updates received by an other handler

```rust no_run
use carapax::Dispatcher;
use carapax_ratelimit::{
    limit_all_chats,
    limit_all_users,
    nonzero,
    DirectRateLimitHandler,
    KeyedRateLimitHandler,
    RateLimitList,
};
use std::time::Duration;

// 1 update per 5 seconds
let (capacity, interval) = (nonzero!(1u32), Duration::from_secs(5));
// limit all updates
let handler = DirectRateLimitHandler::new(capacity, interval);
// Limit updates per user ID for all users
let on_missing = true; // Allow update when key is missing
let handler = KeyedRateLimitHandler::new(
    limit_all_chats,
    on_missing,
    capacity,
    interval,
);
// Limit updates for specific chat or user separately
let handler = KeyedRateLimitHandler::new(
    RateLimitList::default().with_user("username").with_chat("chatusername"),
    on_missing,
    capacity,
    interval,
);
let mut dispatcher = Dispatcher::new(());
dispatcher.add_handler(handler);
// add other handlers here...
```

Note that only handlers added after ratelimit handler will be protected.

### carapax-session

A session utilities

```rust no_run
use carapax::{handler, Api, Command, Config, Dispatcher};
use carapax::longpoll::LongPoll;
use carapax::methods::SendMessage;
use carapax::types::Update;
use carapax_session::{backend::fs::FilesystemBackend, SessionCollector, SessionManager};
use std::{env, time::Duration};
use tempfile::tempdir;

struct Context {
    api: Api,
    session_manager: SessionManager<FilesystemBackend>,
}

#[handler]
async fn handle_update(context: &Context, update: Update) {
    let message = update.get_message().unwrap();
    println!("got a message: {:?}\n", message);
    let chat_id = message.get_chat_id();
    let mut session = context.session_manager.get_session(&update);
    let val: Option<usize> = session.get("counter").await.unwrap();
    let val = val.unwrap_or(0) + 1;
    session.set("counter", &val).await.unwrap();
    let msg = format!("Count: {}", val);
    println!("{}", msg);
    context.api.execute(SendMessage::new(chat_id, msg)).await.unwrap();
}

#[tokio::main]
async fn main() {
    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let api = Api::new(Config::new(token)).expect("Failed to create API");
    let tmpdir = tempdir().expect("Failed to create temp directory");
    println!("Session directory: {}", tmpdir.path().display());
    let backend = FilesystemBackend::new(tmpdir.path());
    // spawn GC to remove old sessions
    let gc_period = Duration::from_secs(1); // period between GC calls
    let session_lifetime = Duration::from_secs(1); // how long session lives
    let mut collector = SessionCollector::new(backend.clone(), gc_period, session_lifetime);
    tokio::spawn(async move { collector.run().await });
    let mut dispatcher = Dispatcher::new(Context {
        api: api.clone(),
        session_manager: SessionManager::new(backend),
    });
    dispatcher.add_handler(handle_update);
    LongPoll::new(api, dispatcher).run().await
}
```

## Examples

In order to run examples you need to create a `.env` file:

```sh
cp sample.env .env
```

Don't forget to change value of `TGRS_TOKEN` and other variables if required.

## Code of Conduct

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## LICENSE

The MIT License (MIT)
