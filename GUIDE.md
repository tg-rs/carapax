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
    // Second - a handler.
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
It should return a future with [`HandlerResult`](https://tg-rs.github.io/carapax/carapax/type.HandlerResult.html) output,
if you want to use it in `App` struct.

`App` creates a [`HandlerInput`](https://tg-rs.github.io/carapax/carapax/struct.HandlerInput.html) with a `Context` and an `Update`,
then converts it into an input for a specific handler using `TryFromInput` trait.
Handler will be executed only when `TryFromInput` returns `Some(T)`.
When your handler is a regular function, `TryFromInput` must return `Some(T)` for all arguments of the function.
Handler will not run otherwise.
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

let chain = Chain::all() // or Chain::once
    .add(handler1)
    .add(handler2)
    .add(handler3);
```

Handlers will run in same order as added.
If a handler returns `Err(_)`, all the subsequent handlers will not run.

- `Chain::all()` - runs all given handlers.
- `Chain::once()` - runs only a first handler found for given input.

### HandlerResult

Normally every handler must return a `HandlerResult` which is alias to `Result` or 
a type that converts into it (e.g. `IntoHandlerResult` implemented for the type):

| From             | To                  |
|------------------|---------------------|
| `()`             | `Ok(())`            |
| `Ok(())`         | `Ok(())`            |
| `Err(YourError)` | `Err(HandlerError)` |

### Error handling

By default, `App` logs an error produced by a handler.

You can use [`ErrorDecorator`](https://tg-rs.github.io/carapax/carapax/struct.ErrorDecorator.html). It allows processing an error returned by a handler.

```rust
use carapax::{
    Chain, ErrorExt, ErrorDecorator, ErrorHandler, HandlerError,
};
use std::num::ParseIntError;
use futures_util::future::BoxFuture;

#[derive(Clone)]
struct LoggingErrorHandler;

impl ErrorHandler for LoggingErrorHandler {
    type Future = BoxFuture<'static, HandlerError>;
    
    fn handle(&self, err: HandlerError) -> Self::Future {
        Box::pin(async {
            log::error!("An error occurred: {}", err);
            err
        })
    }
}

async fn erroneous_handler(_: ()) -> Result<(), ParseIntError> {
    let _num = "not a number".parse::<i32>()?;
    Ok(())
}

fn main() {
    // ...
    // using ErrorExt
    let handler = erroneous_handler.on_error(LoggingErrorHandler);
    // or create decorator by hand
    // let handler = ErrorDecorator::new(LoggingErrorHandler, erroneous_handler);
    let chain = Chain::once().add(handler);
    // ...
}
```

## Predicates

[`Predicate`](https://tg-rs.github.io/carapax/carapax/struct.Predicate.html)
is a decorator which allows deciding, should a handler run or not.
It might be useful if you need to implement a rate-limiter, or allow only certain users to trigger a handler.

A predicate handler must implement `Handler` trait, and return
a [`PredicateResult`](https://tg-rs.github.io/carapax/carapax/enum.PredicateResult.html)
or a type that converts into it:

|   From            | To                            |
|-------------------|-------------------------------|
| `true`            | `PredicateResult::Continue`   |
| `false`           | `PredicateResult::Stop`       |
| `Result<T, E>`    | `T: Into<PredicateResult>`    |


Example:

```rust
use carapax::{
    methods::SendMessage,
    types::{ChatId, Text},
    Api, Chain, Predicate, PredicateExt, Ref,
};

async fn is_ping(text: Text) -> bool {
    text.data == "ping"
}

async fn pong(api: Ref<Api>, chat_id: ChatId) {
    let method = SendMessage::new(chat_id, "pong");
    api.execute(method).await.unwrap();
}

fn main() {
    // ...
    // using PredicateExt
    let handler = pong.predicate(is_ping);
    // or create predicate by hand
    // let handler = Predicate::new(is_ping, pong);
    let chain = Chain::once().add(handler);
    // ...
}
```

### Commands

[`CommandPredicate`](https://tg-rs.github.io/carapax/carapax/struct.CommandPredicate.html#)
is a decorator which allows to run a handler only if update contains a command.
Note that command name contains a leading slash (`/`).

```rust
use carapax::{
    methods::SendMessage,
    types::{ChatId, User},
    Api, Chain, CommandExt, CommandPredicate, Predicate, Ref,
};

async fn greet(api: Ref<Api>, chat_id: ChatId, user: User) {
    let method = SendMessage::new(chat_id, format!("Hello, {}", user.first_name));
    api.execute(method).await.unwrap();
}

fn main() {
    // ...
    // using CommandExt
    let handler = greet.command("/hello");
    // or create predicate by hand
    //let handler = Predicate::new(CommandPredicate::new("/hello"), greet);
    let chain = Chain::once().add(handler);
    // ...
}
```

### Access

[AccessPredicate](https://tg-rs.github.io/carapax/carapax/access/struct.AccessPredicate.html) allows setting up access to handlers.
It takes an [`AccessPolicy`](https://tg-rs.github.io/carapax/carapax/access/trait.AccessPolicy.html).
Policy allows deciding whether access granted or not, depending on `HandlerInput`.

[InMemoryAccessPolicy](https://tg-rs.github.io/carapax/carapax/access/struct.InMemoryAccessPolicy.html) is a policy
which stores [access rules](https://tg-rs.github.io/carapax/carapax/access/struct.AccessRule.html) in memory.

Let's see how it works:

```rust
use carapax::{
    access::{AccessExt, AccessPredicate, AccessRule, InMemoryAccessPolicy},
    types::Update,
    Chain, Predicate, HandlerResult,
};

async fn protected_handler(update: Update) {
    log::info!("Got a new update in protected handler: {:?}", update);
}

fn main() {
    let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user("username")]);
    // using AccessExt
    let handler = protected_handler.access(policy);
    // or create predicate by hand
    // let handler = Predicate::new(AccessPredicate::new(policy), protected_handler);
    let chain = Chain::once().add(handler);
}

```

Since `Chain` implements `Handler` you can also protect a group of handlers.
Naturally, you can implement your own policy in order to store access rules and/or a list of banned users in a database or some other storage.

Note that you need to enable `access` feature in `Cargo.toml`.

### Ratelimit

Carapax provides a ratelimit support using [governor](https://crates.io/crates/governor) crate.

There are two type of predicates 
[`DirectRateLimitPredicate`](https://tg-rs.github.io/carapax/carapax/ratelimit/struct.DirectRateLimitPredicate.html)
and 
[`KeyedRateLimitPredicate`](https://tg-rs.github.io/carapax/carapax/ratelimit/struct.KeyedRateLimitPredicate.html)

Direct is used when you need to apply ratelimit for all incoming updates.
Keyed - when you need to limit updates per chat and/or user.

When limit is reached you can either [discard](https://tg-rs.github.io/carapax/carapax/ratelimit/struct.MethodDiscard.html) the updates,
or [wait](https://tg-rs.github.io/carapax/carapax/ratelimit/struct.MethodWait.html) for the next available time slot.

Every type of predicate can be used [with](https://tg-rs.github.io/carapax/carapax/ratelimit/struct.Jitter.html) or 
[without](https://tg-rs.github.io/carapax/carapax/ratelimit/struct.NoJitter.html) jitter.

See [example](examples/app/ratelimit.rs) for more information.

Same as in access, you can protect a single handler or a chain.

Note that you need to enable `ratelimit` feature in `Cargo.toml`.

## Session


Sessions allow storing data in a storage such as filesystem or redis. 
Therefore you can implement stateful handlers.

Session support is implemented using [seance](http://crates.io/crates/seance) crate.
Carapax reexports all the needed types from seance, so you don't have to add it to your `Cargo.toml`.

Every session has an identifier represented by [SessionId](https://tg-rs.github.io/carapax/carapax/session/struct.SessionId.html) struct.
Note that it contains a chat ID and a user ID, but not all types of update can provide that information.

[`SessionManager`](https://tg-rs.github.io/carapax/carapax/session/struct.SessionManager.html) allows to load a session by ID.
In manager, Session ID represented by a type constrained to `Into<String>`.
If you have issues with updates without chat/user ID just don't use `SessionId`.

You can either get [`Session`](https://tg-rs.github.io/carapax/carapax/session/struct.Session.html) directly from the manager, 
or use `TryFromInput` and specify `session: Session<B>` in handler arguments.
Where `B` is a [session backend](https://tg-rs.github.io/carapax/carapax/session/backend/index.html).
In both cases make sure that session manager is added to the context.

As mentioned above, not all updates has `chat_id` or `user_id`.
And handler will not run if it contains `Session<B>` or `SessionId` in arguments.
In this case you need to get session from manager manually.

See [example](examples/app/session.rs) for more information.

Note that you need to enable either `session-fs` or `session-redis` feature in `Cargo.toml`.

Or just use `session` if you have your own backend.

## Dialogues

Dialogue is a kind of stateful handler. It receives the current state and returns a new one.

[`DialogueDecorator`](https://tg-rs.github.io/carapax/carapax/dialogue/struct.DialogueDecorator.html) allows to make a dialogue handler.
It takes a predicate which allows deciding should we start a dialogue or not, and a handler itself.

Dialogue handler acts like a regular handler but returns a [`DialogueResult`](https://tg-rs.github.io/carapax/carapax/dialogue/enum.DialogueResult.html).
There is a [`DialogueInput`](https://tg-rs.github.io/carapax/carapax/dialogue/struct.DialogueInput.html) structure which allows to obtain a state from the session.
It implements `TryFromInput`, so you can use it as an argument of your handler.

State must implement [`DialogueState`](https://tg-rs.github.io/carapax/carapax/dialogue/trait.DialogueState.html) trait.
Dialogue name must be unique. It defines a value for session key to store the state.

State can be converted into `DialogueResult`.
Thus, you can return `state.into()` instead of `DialogueResult::Next(state)`.

See [example](examples/app/dialogue.rs) for more information.

Note that you need to enable `session` and `dialogue` features in `Cargo.toml`.

And of course you can use any session backend.
