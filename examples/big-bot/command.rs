use crate::util::Module;
use carapax::{
    methods::SendMessage,
    types::{ChatId, User},
    Api, CommandExt, Dispatcher, ExecuteError, HandlerResult, Ref,
};

pub struct CommandModule;

impl Module for CommandModule {
    fn add_handlers(&self, dispatcher: &mut Dispatcher) {
        dispatcher.add_handler(hello_handler.command("/hello"));
    }
}

async fn hello_handler(api: Ref<Api>, chat_id: ChatId, user: User) -> Result<HandlerResult, ExecuteError> {
    let method = SendMessage::new(chat_id, format!("Hello, {}", user.first_name));
    api.execute(method).await?;
    Ok(HandlerResult::Stop)
}
