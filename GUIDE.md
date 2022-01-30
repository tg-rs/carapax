# Introduction

Carapax is based on [tgbot](https://github.com/tg-rs/tgbot) — a core library for interacting with Telegram API.
It provides [`Api`](https://docs.rs/tgbot/latest/tgbot/struct.Api.html) struct
which allows you to execute methods and download files.
And [`UpdateHandler`](https://docs.rs/tgbot/latest/tgbot/trait.UpdateHandler.html) trait to process updates.

Carapax introduces more flexible update handlers and other utilities to build complex bots.

Let's get started:

```rust no_run
use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{ChatId, Text},
    webhook::run_server,
    Api, App, Context, ExecuteError, Ref, SyncedUpdateHandler
};

const DEBUG: bool = true;

async fn echo(api: Ref<Api>, chat_id: ChatId, text: Text) -> Result<(), ExecuteError> {
    let method = SendMessage::new(chat_id, text.data);
    api.execute(method).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Create an api client with a token provided by Bot Father.
    let api = Api::new("BOT_TOKEN").expect("Failed to create API");

    // Context is a type map which allows one to share objects between handlers.
    // Every object you insert in context must implement Clone.
    let mut context = Context::default();
    context.insert(api.clone());

    // App is the main entry point.
    // First argument is the context.
    // Second - an input handler.
    // App implements UpdateHandler trait provided by tgbot,
    // so you can use it in either long polling (`LongPoll::new()`) or webhook (`run_server()`)
    let app = App::new(context, echo);

    if DEBUG {
        // Start receiving updates using longpoll method
        LongPoll::new(api, app).run().await;
    } else {
        // or webhook
        run_server(([127, 0, 0, 1], 8080), "/", SyncedUpdateHandler::new(app)).await.expect("Failed to run webhook");
    }
}
```

## Handlers

In example above `app` calls `echo` handler on every update received from Telegram API.

Handlers must implement [`Handler`](https://tg-rs.github.io/carapax/carapax/trait.Handler.html) trait.
Since this trait is implemented for `Fn` (with up to 10 arguments), we can use regular functions as handlers.
It's recommended to use a function when you need a simple handler.
Implement `Handler` trait for your own type only in complex cases (e.g. you need a custom decorator or predicate).

Handler takes any type that implements [`TryFromInput`](https://tg-rs.github.io/carapax/carapax/trait.TryFromInput.html) as input.
It should return a future with [`HandlerResult`](https://tg-rs.github.io/carapax/carapax/enum.HandlerResult.html) output,
if you want to use it in `App` struct.

`App` creates a [`HandlerInput`](https://tg-rs.github.io/carapax/carapax/struct.HandlerInput.html) with a `Context` and an `Update`,
then converts it into an input for a specific handler using `TryFromInput` trait.
Handler will be executed only when `TryFromInput` returns `Some(T)`.
So, in example above, `echo` will be called only when `Update` contains any `Text` (message text, media captions).

Use `Ref<T>` as the argument type when you need to get an object from the context.
If object is not found, `TryFromInput` returns an error, therefore the handler will not run.

### Chain

Chain is a handler which allows to execute several handlers, one after another:

```rust
use carapax::Chain;

async fn handler1(_: ()) {}
async fn handler2(_: ()) {}
async fn handler3(_: ()) {}

let chain = Chain::default()
    .add_handler(handler1)
    .add_handler(handler2)
    .add_handler(handler3);
```

Handlers will run in same order as added.
If a handler returns `HandlerResult::Stop` or `HandlerResult::Error(_)`, all the subsequent handlers will not run.

### HandlerResult

Normally every handler must return a `HandlerResult` or a type that converts into it:

| From            | To                          |
|-----------------|-----------------------------|
| `()`            | `HandlerResult::Stop`       |
| `true`          | `HandlerResult::Continue`   |
| `false`         | `HandlerResult::Stop`       |
| `Result<T, E>`  | `T.into::<HandlerResult>()` |


## Predicates

[`Predicate`](https://tg-rs.github.io/carapax/carapax/struct.Predicate.html)
is a decorator which allows to decide, should a handler run or not.
It might be useful if you need to implement a ratelimiter, or allow only certain users to trigger a handler.

A predicate handler must implement `Handler` trait, and return
a [`PredicateResult`](https://tg-rs.github.io/carapax/carapax/enum.PredicateResult.html)
or a type that converts into it:

|   From            | To                            |
|-------------------|-------------------------------|
| `true`            | `PredicateResult::Continue`   |
| `false`           | `PredicateResult::Stop`       |
| `Result<T, E>`    | `T.into::<PredicateResult>()` |


Example:

```rust
use carapax::{
    methods::SendMessage,
    types::{ChatId, Text},
    Api, Chain, PredicateExt, Ref,
};

fn setup(chain: &mut Chain) {
    let handler = pong.predicate(is_ping);
    chain.add_handler(handler);
}

async fn is_ping(text: Text) -> bool {
    text.data == "ping"
}

async fn pong(api: Ref<Api>, chat_id: ChatId) {
    let method = SendMessage::new(chat_id, "pong");
    api.execute(method).await.unwrap();
}

```

### Commands

[`CommandPredicate`](https://tg-rs.github.io/carapax/carapax/struct.CommandPredicate.html#)
is a decorator which allows to run handlers only when user sent a command.
Note that command name contains a leading slash (`/`).

```rust
use carapax::{
    methods::SendMessage,
    types::{ChatId, User},
    Api, Chain, CommandExt, Ref,
};

fn setup(chain: &mut Chain) {
    let handler = greet.command("/hello");
    chain.add_handler(handler);
}

async fn greet(api: Ref<Api>, chat_id: ChatId, user: User) {
    let method = SendMessage::new(chat_id, format!("Hello, {}", user.first_name));
    api.execute(method).await.unwrap();
}

```

### Access

[AccessPredicate](https://tg-rs.github.io/carapax/carapax/access/struct.AccessPredicate.html) allows to protect handlers from unwanted users.
It takes an [`AccessPolicy`](https://tg-rs.github.io/carapax/carapax/access/trait.AccessPolicy.html).
Policy has `is_granted` method which takes a `HandlerInput` and returns a future with `bool` output.
If `true` is returned - access is granted, `false` - forbidden.

[InMemoryAccessPolicy](https://tg-rs.github.io/carapax/carapax/access/struct.InMemoryAccessPolicy.html) is a simple policy 
which stores [access rules](https://tg-rs.github.io/carapax/carapax/access/struct.AccessRule.html) in memory.

Let's see how it works:

```rust
use carapax::{
    access::{AccessExt, AccessPredicate, AccessRule, InMemoryAccessPolicy},
    types::Update,
    Chain, Predicate, HandlerResult,
};

fn setup(chain: &mut Chain) {
    let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user("username")]);
    let handler = Predicate::new(AccessPredicate::new(policy), protected_handler);
    // or using AccessExt
    // let handler = protected_handler.access(policy);
    chain.add_handler(handler);
}

async fn protected_handler(update: Update) {
    log::info!("Got a new update in protected handler: {:?}", update);
}

```

Since `Chain` implements `Handler` you can also protect a group of handlers or entire bot.
Naturally, you can implement your own policy in order to store the access rules and/or a list of banned users in a database or some other storage.

Note that you need to enable `access` feature in `Cargo.toml`:

```toml
carapax = { version = "0.11.0", features = ["access"] }
```


### Ratelimit

Carapax provides a ratelimit support using [governor](https://crates.io/crates/governor) crate.

There are two type of predicates 
[`DirectRateLimitPredicate`](https://tg-rs.github.io/carapax/carapax/ratelimit/struct.DirectRateLimitPredicate.html)
and 
[`KeyedRateLimitPredicate`](https://tg-rs.github.io/carapax/carapax/ratelimit/struct.KeyedRateLimitPredicate.html)

Direct is used when you need to limit all updates. Keyed - when you need to limit updates per chat and/or user.

When limit is reached you can either [discard](https://tg-rs.github.io/carapax/carapax/ratelimit/struct.MethodDiscard.html) the updates,
or [wait](https://tg-rs.github.io/carapax/carapax/ratelimit/struct.MethodWait.html) when next limiter will allow to pass an update

Every type of predicate can be used [with](https://tg-rs.github.io/carapax/carapax/ratelimit/struct.Jitter.html) or 
[without](https://tg-rs.github.io/carapax/carapax/ratelimit/struct.NoJitter.html) jitter.

See [example](examples/app/ratelimit.rs) for implementation details.

Same as in access, you can protect a single handler, a chain or the entire bot.

Note that you need to enable `ratelimit` feature in `Cargo.toml`:

```toml
carapax = { version = "0.11.0", features = ["ratelimit"] }
```

## Session


Sessions allow to store data in a storage such as filesystem or redis. 
Therefore you can implement stateful handlers.

Session support is implemented using [seance](http://crates.io/crates/seance) crate.
Carapax reexports all the needed types from seance, so you don't have to add it to your `Cargo.toml`.

Every session has an identifier represented by [SessionId](https://tg-rs.github.io/carapax/carapax/session/struct.SessionId.html) struct.
Note that it contains a chat ID and an user ID, but not all types of update can provide that information.

[`SessionManager`](https://tg-rs.github.io/carapax/carapax/session/struct.SessionManager.html) allows to load a session by ID.
Session ID in manager represented by `Into<String>` so you can use any type you want, if you have issues with updates without chat/user ID.

You can either get [`Session`](https://tg-rs.github.io/carapax/carapax/session/struct.Session.html) directly from the manager, 
or use `TryFromInput` and specify `session: Session<B>` in handler arguments.
In both cases make sure that session manager is added to the context.

See [example](examples/app/session.rs) for implementation details.

Note that you need to enable either `session-fs` or `session-redis` feature in `Cargo.toml`:

```toml
carapax = { version = "0.11.0", features = ["session-fs"] }
```

Or simply just use `session` if you have your own backend.

## Dialogues

Dialogue is a kind of stateful handler. It receives an initial or previous state and returns a new state.

[`DialogueDecorator`](https://tg-rs.github.io/carapax/carapax/dialogue/struct.DialogueDecorator.html) allows to make a dialogue handler.
It takes a predicate which allows to decide should we start a dialogue or not, and a handler itself.

Dialogue handler takes a `HandlerInput` and returns a [`DialogueResult`](https://tg-rs.github.io/carapax/carapax/dialogue/enum.DialogueResult.html).
There is a [`DialogueInput`](https://tg-rs.github.io/carapax/carapax/dialogue/struct.DialogueInput.html) structure which allows to obtain a state from the session.
It implements `TryFromInput`, so you can use it as an argument of your handler.

[`DialogueState`](https://tg-rs.github.io/carapax/carapax/dialogue/trait.DialogueState.html) is a trait that you must implement for your own state.
Dialogue name is used to determine the session key for that dialogue, and must be unique.

When returning a state you can convert it into `DialogueResult` without using `DialogueResult::Next(state)`: `state.into()`.

See [example](examples/app/dialogue.rs) for implementation details.

Note that you need to enable `session` and `dialogue` features in `Cargo.toml`:

```toml
carapax = { version = "0.11.0", features = ["session-fs", "dialogue"] }
```

And of course you can use any session backend.
