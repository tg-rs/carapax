# CARAPAX

[![CI](https://img.shields.io/github/workflow/status/tg-rs/carapax/CI?style=flat-square)](https://github.com/tg-rs/carapax/actions/)
[![Codecov](https://img.shields.io/codecov/c/github/tg-rs/carapax.svg?style=flat-square)](https://codecov.io/gh/tg-rs/carapax)
[![Version](https://img.shields.io/crates/v/carapax.svg?style=flat-square)](https://crates.io/crates/carapax)
[![Downloads](https://img.shields.io/crates/d/carapax.svg?style=flat-square)](https://crates.io/crates/carapax)
[![Release Documentation](https://img.shields.io/badge/docs-release-brightgreen.svg?style=flat-square)](https://docs.rs/carapax)
[![Master Documentation](https://img.shields.io/badge/docs-master-blueviolet.svg?style=flat-square)](https://tg-rs.github.io/carapax/carapax/)
[![Telegram Chat](https://img.shields.io/badge/telegram-@tgrsusers-blue?style=flat-square)](https://t.me/tgrsusers)
[![License](https://img.shields.io/crates/l/carapax.svg?style=flat-square)](https://github.com/tg-rs/carapax/tree/master/LICENSE)

A telegram bot framework based on [tgbot](https://github.com/tg-rs/tgbot).

Name comes from [Carapace](https://en.wikipedia.org/wiki/Carapace) (carapax in latin).

## Installation

```toml
[dependencies]
carapax = "0.9.0"
```

## Examples

In order to run [examples](https://github.com/tg-rs/carapax/tree/master/carapax/examples) you need to create a `.env` file:

```sh
cp sample.env .env
```

Don't forget to change value of `CARAPAX_TOKEN` and other variables if required.

## Introduction

```rust no_run
#[tokio::main]
async fn main() {

// Setup an API client:
use carapax::{Api, Config};

let api = Api::new("bot-token").unwrap();
// Or:
let config = Config::new("bot-token").host("custom-api-host").proxy("proxy-url").unwrap();
let api = Api::new(config).unwrap();

// And dispatcher:
use carapax::Dispatcher;

// Dispatcher takes a context which will be passed to each handler
// we use api client for this, but you can pass any struct.
let mut dispatcher = Dispatcher::new(api.clone());

// Let's add a command handler
use carapax::{types::Command, handler};

#[handler(command = "/start")]
async fn command_handler(_context: &Api, _command: Command) {
    // handler takes a reference to context passed to dispatcher
}

dispatcher.add_handler(command_handler);

// A message handler:
use carapax::{types::Message, HandlerResult};

#[handler]
async fn message_handler(_context: &Api, _message: Message) -> HandlerResult {
    // handle message here...

    // say that next handler will run
    HandlerResult::Continue
    // but you can prevent next handler by using HandlerResult::Stop
    // or return an error using `HandlerResult::Error`: Err(err).into()
    // In case of error, next handler will not run by default. See below how to change this behavior.
}

dispatcher.add_handler(message_handler);

// You also can implement Handler for a struct:
struct UpdateHandler;

use carapax::{async_trait, Handler, ExecuteError};
use carapax::methods::SendMessage;
use carapax::types::Update;

// note: #[handler] macro expands to something like this
#[async_trait]
impl Handler<Api> for UpdateHandler {
    // An object to handle (update, message, inline query, etc...)
    type Input = Update;
    // A result to return
    // You can use Result<T, E>, HandlerResult or ()
    type Output = Result<(), ExecuteError>;

    async fn handle(&mut self, context: &Api, input: Self::Input) -> Self::Output {
        if let Some(chat_id) = input.get_chat_id() {
            context.execute(SendMessage::new(chat_id, "Hello!")).await?;
        }
        Ok(())
    }
}

dispatcher.add_handler(UpdateHandler);

// in order to catch errors occurred in handlers you can set an error hander:

use carapax::{ErrorHandler, LoggingErrorHandler, ErrorPolicy, HandlerError};

// log error and go to the next handler
dispatcher.set_error_handler(LoggingErrorHandler::new(ErrorPolicy::Continue));
// by default dispatcher logs error and stops update propagation (next handler will not run)

// or you can implement your own error handler:

struct MyErrorHandler;

#[async_trait]
impl ErrorHandler for MyErrorHandler {
    async fn handle(&mut self, err: HandlerError) -> ErrorPolicy {
        ErrorPolicy::Continue
    }
}

dispatcher.set_error_handler(MyErrorHandler);

// now you can start your bot:

// using long polling
use carapax::longpoll::LongPoll;


LongPoll::new(api, dispatcher).run().await;

// or webhook
// use carapax::webhook::run_server;
// run_server(([127, 0, 0, 1], 8080), "/path", dispatcher).await.unwrap();

}
```

### Access rules

Carapax provides an access handler which allows you to protect your handlers from unwanted users.

To use this handler you need to enable `access` feature in Cargo.toml:

```toml
[dependencies]
carapax = { version = "*", features=["access"] }
```

```rust no_run
use carapax::Dispatcher;
use carapax::access::{AccessHandler, AccessRule, InMemoryAccessPolicy};

// Deny from all except for @username (specify without @)
let rule = AccessRule::allow_user("username");
// See API documentation for more information about rules
let policy = InMemoryAccessPolicy::default().push_rule(rule);

// Also you can implement your own access policy:
use carapax::{access::AccessPolicy, types::Update, async_trait};

struct MyAccessPolicy;

#[async_trait]
impl AccessPolicy<()> for MyAccessPolicy
{
    async fn is_granted(&mut self, _context: &(), update: &Update) -> bool {
        true
    }
}


let mut dispatcher = Dispatcher::new(());
dispatcher.add_handler(AccessHandler::new(policy));
// Add other handlers here..
// Note that you should add access handler before any other handlers you want to protect.

```

### Dialogues

You can easily implement dialogues by enabling `dialogue` and [`session-fs` or `session-redis`] features:

```rust no_run
use carapax::{
    Dispatcher,
    session::{backend::fs::FilesystemBackend, SessionManager},
    dialogue::{
        Dialogue,
        DialogueResult::{self, *},
        State,
        dialogue
    },
    types::Message
};
use std::convert::Infallible;
use serde::{Serialize, Deserialize};
use tempfile::tempdir;

// First we describe dialogue state

#[derive(Serialize, Deserialize)]
enum ExampleState {
    Start,
    Step1,
    Step2,
}

impl State for ExampleState {
    // Returns initial state
    fn new() -> Self {
        ExampleState::Start
    }
}

// A special dialogue handler which takes an old state and returns a new state
#[dialogue]
async fn my_dialogue(
    state: ExampleState,
    context: &(),
    input: Message,
) -> Result<DialogueResult<ExampleState>, Infallible> {
    Ok(match state {
        ExampleState::Start => {
            Next(ExampleState::Step1)
        },
        ExampleState::Step1 => {
            Next(ExampleState::Step2)
        }
        ExampleState::Step2 => {
            Exit
        }
    })
}

let tmpdir = tempdir().expect("Failed to create temp directory");
let session_backend = FilesystemBackend::new(tmpdir.path());
let session_manager = SessionManager::new(session_backend);
let mut dispatcher = Dispatcher::new(());
let dialogue_name = "example";  // unique dialogue name used to store state
// `Dialogue` is responsible for loading and saving state
let handler = Dialogue::new(session_manager, dialogue_name, my_dialogue);
dispatcher.add_handler(handler);
```

### Internationalization

Carapax has i18n support provided by [gettext](https://www.gnu.org/software/gettext/).

Note that you should enable `i18n` feature in `Cargo.toml`.

```rust no_run
use carapax::{handler, methods::SendMessage, types::Update, Api};
use carapax::i18n::{Catalog, Translator, TranslatorStore};

const RU: &[u8] = include_bytes!("../../carapax/data/ru.mo");
const EN: &[u8] = include_bytes!("../../carapax/data/en.mo");

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
```

### Ratelimit

You can limit number of updates received by a handler.

Note that you should enable `ratelimit` feature in `Cargo.toml`.

```rust no_run
use carapax::Dispatcher;
use carapax::ratelimit::{
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

### Session

Sessions support provided by [seance](https://github.com/rossnomann/seance) crate.

You should enable `session-fs` feature if you want to use filesystem based backend
or `session-redis` for redis backend.
Also you can specify `session` feature if you have a custom backend.

```rust no_run
use carapax::{handler, Api, Dispatcher};
use carapax::methods::SendMessage;
use carapax::types::Update;
use carapax::session::{backend::fs::FilesystemBackend, SessionCollector, SessionManager};
use std::time::Duration;
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
    // get user session from update
    let mut session = context.session_manager.get_session(&update).unwrap();
    // get a value from session
    let val: Option<usize> = session.get("counter").await.unwrap();
    let val = val.unwrap_or(0) + 1;
    // set a value to session
    session.set("counter", &val).await.unwrap();
    let msg = format!("Count: {}", val);
    println!("{}", msg);
    context.api.execute(SendMessage::new(chat_id, msg)).await.unwrap();
}

let tmpdir = tempdir().expect("Failed to create temp directory");
let backend = FilesystemBackend::new(tmpdir.path());

// spawn GC to remove old sessions
let gc_period = Duration::from_secs(5); // period between GC calls
let session_lifetime = Duration::from_secs(86400 * 7); // how long session lives
let mut collector = SessionCollector::new(backend.clone(), gc_period, session_lifetime);
tokio::spawn(async move { collector.run().await });

let api = Api::new("token").unwrap();
let mut dispatcher = Dispatcher::new(Context {
    api: api.clone(),
    session_manager: SessionManager::new(backend),
});
dispatcher.add_handler(handle_update);
```

# Changelog

See [CHANGELOG.md](https://github.com/tg-rs/carapax/tree/master/CHANGELOG.md)

## Code of Conduct

See [CODE_OF_CONDUCT.md](https://github.com/tg-rs/carapax/tree/master/CODE_OF_CONDUCT.md).

## LICENSE

The MIT License (MIT)
