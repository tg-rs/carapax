//! The example demonstrates the basic usage of the framework.
//!
//! The following steps are required to run a simple bot:
//!
//! 1. Declare an [`Update`] handler.
//! 2. Obtain a Bot API token.
//! 3. Create an API [`Client`] instance to interact with the Telegram Bot API.
//! 4. Create a [`Context`] instance to share objects with the handler.
//! 5. Create an [`App`] instance which serves as the main entry point for the bot.
//! 6. Start the bot using the [`LongPoll`] for development environment
//!    or the [`WebhookServer`] for production environment.
//!
//! The [`App`] creates a [`HandlerInput`] containing the [`Context`] instance
//! and an [`Update`] received from the Telegram Bot API,
//! then converts it into an input for the handler using [`TryFromInput`] trait.
//!
//! The handler must implement the [`Handler`] trait.
//! Since this trait is implemented for [`Fn`] (with up to 10 arguments),
//! you can use regular functions as handlers.
//!
//! It's recommended to use a function when you need a simple handler.
//! Implement the [`Handler`] trait for your own type only in complex cases,
//! e.g. you need to write your own decorator or a predicate.
//!
//! The handler is executed only when the [`TryFromInput`] returns `Some(T)`.
//! When your handler is a regular function, [`TryFromInput`] must return `Some(T)`
//! for all arguments of the function. Otherwise the handler will not be executed.
//!
//! The handler must return a future with a [`HandlerResult`] output.
//! You can use any type that converts into it
//! ([`IntoHandlerResult`] is implemented for the type):
//!
//! | From             | To                  |
//! |------------------|---------------------|
//! | `()`             | `Ok(())`            |
//! | `Ok(())`         | `Ok(())`            |
//! | `Err(YourError)` | `Err(HandlerError)` |
//!
//! See the `app` example for advanced usage information.
//!
//! [`IntoHandlerResult`]: carapax::IntoHandlerResult
//! [`Handler`]: carapax::Handler
//! [`HandlerInput`]: carapax::HandlerInput
//! [`HandlerResult`]: carapax::HandlerResult
//! [`TryFromInput`]: carapax::TryFromInput
//! [`Update`]: carapax::types::Update

use std::env;

use dotenvy::dotenv;

use carapax::{
    api::{Client, ExecuteError},
    handler::{LongPoll, WebhookServer},
    types::{ChatPeerId, SendMessage, Text},
    App, Context, Ref,
};

const DEBUG: bool = true;

/// The update handler
///
/// It will be executed only when the [`carapax::types::Update`] contains
/// a [`ChatPeerId`] and a [`Text`] (message text, media captions).
///
/// Use [`Ref`] as the argument type when you need to retrieve an object from the context.
/// If the object is not found, [`carapax::TryFromInput`] returns an error and
/// the handler will not be executed.
async fn echo(client: Ref<Client>, chat_id: ChatPeerId, message: Text) -> Result<(), ExecuteError> {
    let method = SendMessage::new(chat_id, message.data);
    client.execute(method).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    // Create an api client with a token provided by Bot Father.
    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let client = Client::new(token).expect("Failed to create API");

    // Context is a type map that enables the sharing of objects with handlers.
    // Each object inserted into the context must implement the `Clone` trait.
    let mut context = Context::default();
    context.insert(client.clone());

    let app = App::new(context, echo);

    if DEBUG {
        // Start receiving updates using long polling method
        LongPoll::new(client, app).run().await
    } else {
        // or webhook
        WebhookServer::new("/", app)
            .run(([127, 0, 0, 1], 8080))
            .await
            .expect("Failed to run webhook");
    }
}
