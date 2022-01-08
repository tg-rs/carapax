use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{ChatId, Text},
    Api, Context, Dispatcher, ExecuteError, Ref,
};

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Text) -> Result<(), ExecuteError> {
    let method = SendMessage::new(chat_id, message.data);
    api.execute(method).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let api = Api::new("YOUR_TOKEN").expect("Failed to create API");

    let mut context = Context::default();
    context.insert(api.clone());

    let mut dispatcher = Dispatcher::new(context);
    dispatcher.add_handler(echo);

    LongPoll::new(api, dispatcher).run().await
}
