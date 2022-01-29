# Simple app

Here is a simple app:

```rust
use carapax::{Api, Context, Api, Ref};
use carapax::types::{ChatId, Text};
use carapax::methods::SendMessage;
use carapax::longpoll::LongPoll;

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Text) -> Result<(), ExecuteError> {
    let method = SendMessage::new(chat_id, message.data);
    api.execute(method).await?;
    Ok(())
}

let api = Api::new("BOT_TOKEN")?;

let context = Context::new();
context.insert(api.clone());

let app = App::new(context, echo);
LongPoll::new(api, app).run().await;
```

Let's figure it out line-by-line.

```rust
let api = Api::new("BOT_TOKEN")?;
```

`Api` is something like you can usually see as `Bot` in other languages. It executes all API methods and download files.

```rust
let context = Context::new();
context.insert(api.clone());
```

`Context` is type map which contains all any objects you insert. We add `Api` into it in this case.

```rust
let app = App::new(context, echo);
```

`App` is the main entry point, the link between context and handler (`echo`). App implements `UpdateHandler` trait
which is used for handling updates Telegram sends you.

```rust
LongPoll::new(api, app).run().await;
```

`LongPoll` is structure for receiving updates using long polling. It accepts `Api` to execute `getUpdates` method and `UpdateHandler` which handle updates.

Now the most interesting part. 

```rust
fn echo(api: Ref<Api>, chat_id: ChatId, message: Text) -> Result<(), ExecuteError> {
    let method = SendMessage::new(chat_id, message.data);
    api.execute(method).await?;
    Ok(())
}
```

`echo` is our handler. Signature of the handler also defines it propagation behavior.

Every argument of `echo` implements `TryFromInput` so it creates itself from Telegram's update and `Context`.

```rust
api: Ref<Api>
```

We get our data from `Context` using `Ref` type

```rust
chat_id: ChatId
```

We want to get chat ID so there will be try to get it from update. 
If it is not applicable, handler will not be executed.

```rust
message: Text
```

Similar to the previous argument.

```rust
let method = SendMessage::new(chat_id, message.data);
api.execute(method).await?;
```

We execute `sendMessage` method.

# `Chain` handler

What if we have bot that is more complicated than just sending messages back?

We can use `Chain` handler. It chains handlers and run them in a sequence.

```rust
let chain = Chain::default();
chain
    .add_handler(rate_limiter)
    .add_handler(statistics)
    .add_handler(cas)
    .add_handler(start.command("/start"))
    .add_handler(info.command("/info")) 
    .add_handler(cats);
```

# Controlling propagation

Propagation of handlers can be controlled in two different ways:
1. By handler arguments. 

For example:

```rust
fn i_want_text(message: Text) {}
```

We expect message text but if user sends sticker, handler will not run and propagation of the next one will be started.

2. Return `HandlerResult`.

```rust
async fn cas(client: Ref<CasClient>, user: User) -> HandlerResult {
    let is_user_banned = client.check_user(user.id);
    if is_user_banned {
        HandlerResult::Stop
    } else {
        HandlerResult::Continue
    }
} 
```

In case of ban any next handler will not run.

# `*Ext` traits

It's recommended to use traits like `CommandExt`, `PredicateExt`, etc for convenient usage.

For example:

```rust
use carapax::PredicateExt;

let handler = pong.predicate(is_ping); 
```

vs

```rust
let handler = Predicate::new(is_ping, pong);
```

# Implement `TryFromInput` for your own types

Just implement this trait:

```rust
pub trait TryFromInput: Send + Sized {
    type Future: Future<Output = Result<Option<Self>, Self::Error>> + Send;
    type Error: Error + Send;
    fn try_from_input(input: HandlerInput) -> Self::Future;
}
```

What you should return in different cases:
* `Ok(Some(...))` in case of successful type creation.
* `Ok(None)` in case of type cannot be created, and it is not critical. Handler will not run.

For example, implementation of `Text` will return `None` if `Update` does not contain any actual text.

* `Err(...)` in case of error during type creation.

# `access` module

```rust
use carapax::access::{InMemoryAccessPolicy, AccessRule};

let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user("john")]);
let handler = image_handler.access(policy);
```

We create `InMemoryAccessPolicy` structure with rule to run `image_handler` only if message is from `@john`.

It implements `AccessPolicy` trait which `access` method (from `AccessExt` trait) wait as the argument.

# `session` module

Session works with data from persistent store, so you can be sure your data is not lost across, say, bot restarts.

```rust
use carapax::session::{FilesystemBackend, SessionManager, Session, SessionError};

let tmpdir = tempdir()?;
let backend = FilesystemBackend::new(tmpdir.path());

let gc_period = Duration::from_secs(120); // period between GC calls
let session_lifetime = Duration::from_secs(60 * 60); // how long session lives
// spawn GC to remove old sessions
let mut collector = SessionCollector::new(backend.clone(), gc_period, session_lifetime);
tokio::spawn(async move { collector.run().await });

let manager = SessionManager::new(backend);
context.insert(manager);

fn store_data(session: Session<FilesystemBackend>) -> Result<(), SessionError> {
    session.set("test", 123)
}
context.insert(store_data);
```

`backend` responds for session storing. Filesystem in this example.

`manager` will be used by `Session` to acquire itself.

# `dialogue` module

See [dialogue.rs](examples/app/dialogue.rs) for example usage.

The main thing you must know dialogues need session manager for their states.

# `ratelimit` module

```rust
use carapax::ratelimit::{nonzero, DirectRateLimitPredicate, Jitter, Quota};

let quota = Quota::with_period(Duration::from_secs(5))
    .expect("Failed to create quota")
    .allow_burst(nonzero!(1u32));
let jitter = Jitter::up_to(Duration::from_secs(5));

let handler = my_handler.predicate(DirectRateLimitPredicate::wait_with_jitter(quota, jitter));
```

As you can see, ratelimit is applied through predicate.
