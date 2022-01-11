use crate::util::Module;
use carapax::{
    methods::SendMessage,
    types::{Message, Text},
    Api, Dispatcher, ExecuteError, HandlerResult, PredicateExt, Ref,
};

pub struct PingPongModule;

impl Module for PingPongModule {
    fn add_handlers(&self, dispatcher: &mut Dispatcher) {
        dispatcher.add_handler(pingpong_handler.predicate(is_ping));
    }
}

async fn is_ping(text: Text) -> bool {
    text.data == "ping"
}

async fn pingpong_handler(api: Ref<Api>, message: Message) -> Result<HandlerResult, ExecuteError> {
    let chat_id = message.get_chat_id();
    let method = SendMessage::new(chat_id, "pong");
    api.execute(method).await?;
    Ok(HandlerResult::Stop)
}
