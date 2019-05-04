#[cfg(feature = "fs-store")]
fn main() {
    use carapax::prelude::*;
    use carapax_session::{spawn_gc, store::fs::FsSessionStore, Session, SessionHandler};
    use dotenv::dotenv;
    use futures::{future, Future};
    use std::{env, time::Duration};

    fn handle_set(context: &mut Context, message: &Message, args: Vec<String>) -> HandlerFuture {
        log::info!("got a message: {:?}\n", message);
        let session = context.get::<Session<FsSessionStore>>().clone();
        let api = context.get::<Api>().clone();
        let chat_id = message.get_chat_id();
        let val = if args.is_empty() {
            0
        } else {
            match args[0].parse::<usize>() {
                Ok(x) => x,
                Err(err) => {
                    return HandlerFuture::new(
                        api.execute(SendMessage::new(chat_id, err.to_string()))
                            .and_then(|_| Ok(HandlerResult::Stop)),
                    );
                }
            }
        };
        HandlerFuture::new(session.set("counter", &val).and_then(move |()| {
            api.execute(SendMessage::new(chat_id, "OK"))
                .and_then(|_| Ok(HandlerResult::Stop))
        }))
    }

    fn handle_expire(context: &mut Context, message: &Message, args: Vec<String>) -> HandlerFuture {
        log::info!("got a message: {:?}\n", message);
        let session = context.get::<Session<FsSessionStore>>().clone();
        let api = context.get::<Api>().clone();
        let chat_id = message.get_chat_id();
        let seconds = if args.is_empty() {
            0
        } else {
            match args[0].parse::<usize>() {
                Ok(x) => x,
                Err(err) => {
                    return HandlerFuture::new(
                        api.execute(SendMessage::new(chat_id, err.to_string()))
                            .and_then(|_| Ok(HandlerResult::Stop)),
                    );
                }
            }
        };
        HandlerFuture::new(session.expire("counter", seconds).and_then(move |()| {
            api.execute(SendMessage::new(chat_id, "OK"))
                .and_then(|_| Ok(HandlerResult::Stop))
        }))
    }

    fn handle_reset(context: &mut Context, message: &Message, _args: Vec<String>) -> HandlerFuture {
        log::info!("got a message: {:?}\n", message);
        let session = context.get::<Session<FsSessionStore>>().clone();
        let api = context.get::<Api>().clone();
        let chat_id = message.get_chat_id();
        HandlerFuture::new(session.del("counter").and_then(move |()| {
            api.execute(SendMessage::new(chat_id, "OK"))
                .and_then(|_| Ok(HandlerResult::Stop))
        }))
    }

    fn handle_message(context: &mut Context, message: Message) -> HandlerFuture {
        log::info!("got a message: {:?}\n", message);
        let session = context.get::<Session<FsSessionStore>>().clone();
        let api = context.get::<Api>().clone();
        let chat_id = message.get_chat_id();
        HandlerFuture::new(session.get::<usize>("counter").and_then(move |val| {
            let val = val.unwrap_or(0) + 1;
            session.set("counter", &val).and_then(move |()| {
                api.execute(SendMessage::new(chat_id, format!("Count: {}", val)))
                    .and_then(|_| Ok(HandlerResult::Continue))
            })
        }))
    }

    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy);
    }

    let api = Api::new(config).unwrap();
    let commands = CommandsHandler::default()
        .add_handler("/set", FnCommandHandler::from(handle_set))
        .add_handler("/reset", FnCommandHandler::from(handle_reset))
        .add_handler("/expire", FnCommandHandler::from(handle_expire));
    tokio::run(future::lazy(|| {
        FsSessionStore::open("/tmp/carapax-session")
            .map_err(|e| log::error!("Failed to create session store: {:?}", e))
            .and_then(|mut store| {
                store = store.with_lifetime(10);
                spawn_gc(Duration::from_secs(10), store.clone());
                App::new()
                    .add_handler(SessionHandler::new(store))
                    .add_handler(commands)
                    .add_handler(FnHandler::from(handle_message))
                    .run(api.clone(), UpdateMethod::poll(UpdatesStream::new(api)))
            })
    }));
}

#[cfg(not(feature = "fs-store"))]
fn main() {
    println!(r#"Please enable "fs-store" feature"#);
}
