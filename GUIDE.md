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

`Context` is type map which contains all user data. We add `Api` into it in this case.

```rust
let app = App::new(context, echo);
```

`App` is the main entry point, the link between context and handler (`echo`). App implements `UpdateHandler` trait
which is used for handling updates Telegram send you.

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

`echo` is our handler. Signature of the handler also define it propagation behavior.

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

We can use `Chain` handler. It chains handler and run them in a sequence.

```rust
let chain = Chain::default();
chain
    .add_handler(rate_limiter)
    .add_handler(statistics)
    .add_handler(cas)
    .add_handler(start) // let's say this handler works with `/start` command
    .add_handler(info) // this one shows information on `/info` command
    .add_handler(cats); // and this one sends you awesome cats
```

# Controlling propagation

Propagation of handlers can be controlled in two different ways:
1. By handler arguments. 

For example:

```rust
fn i_want_text(message: Text) {}
```

We expect message text but if user sends video, handler will not run and propagation of the next one will be started.

2. Return `HandlerResult`.

```rust
async fn cas(client: Ref<CasClient>, user: User) -> HandlerResult {
    let is_user_banned = client.check_user(user.id);
    if is_user_banned {
        HandlerResult::Continue
    } else {
        HandlerResult::Start
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
