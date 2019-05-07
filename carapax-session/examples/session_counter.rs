#[cfg(not(any(feature = "fs-store", feature = "redis-store")))]
pub fn main() {
    println!(r#"Please enable one of features: "fs-store", "redis-store""#);
}

#[cfg(all(feature = "fs-store", feature = "redis-store"))]
pub fn main() {
    println!(r#"Do not enable both features "fs-store" and "redis-store""#);
}

#[cfg(all(
    any(feature = "fs-store", feature = "redis-store"),
    not(all(feature = "fs-store", feature = "redis-store"))
))]
pub fn main() {
    use carapax::prelude::*;
    use carapax_session::SessionHandler;
    use dotenv::dotenv;
    use futures::Future;
    use std::{env, num::ParseIntError};
    use support::{create_store, Session};

    #[cfg(feature = "fs-store")]
    mod support {
        use carapax_session::{spawn_gc, store::fs::FsSessionStore, Session as CarapaxSession};
        use failure::Error;
        use futures::Future;
        use std::time::Duration;

        #[cfg(target_os = "windows")]
        const TEMP_DIR: &str = env!("TEMP");

        #[cfg(not(target_os = "windows"))]
        const TEMP_DIR: &str = "/tmp";

        pub type Session = CarapaxSession<FsSessionStore>;

        pub fn create_store() -> impl Future<Item = FsSessionStore, Error = Error> {
            FsSessionStore::open(format!("{}/{}", TEMP_DIR, "carapax-session")).map(|mut store| {
                store = store.with_lifetime(10);
                spawn_gc(Duration::from_secs(10), store.clone());
                store
            })
        }
    }

    #[cfg(feature = "redis-store")]
    mod support {
        use carapax_session::{store::redis::RedisSessionStore, Session as CarapaxSession};
        use failure::Error;
        use futures::Future;
        use std::env;

        pub type Session = CarapaxSession<RedisSessionStore>;

        pub fn create_store() -> impl Future<Item = RedisSessionStore, Error = Error> {
            let redis_url = env::var("TGRS_REDIS_URL").expect("TGRS_REDIS_URL is not set");
            RedisSessionStore::open(redis_url, "carapax-session").map(|store| store.with_lifetime(10))
        }
    }

    fn parse_args(args: Vec<String>) -> Result<usize, ParseIntError> {
        if args.is_empty() {
            Ok(0)
        } else {
            args[0].parse::<usize>()
        }
    }

    fn log_message(_context: &mut Context, message: Message) {
        log::info!("Got a message: {:?}\n", message);
    }

    fn set(context: &mut Context, message: Message, args: Vec<String>) -> HandlerFuture {
        let session = context.get::<Session>().clone();
        let api = context.get::<Api>().clone();
        let chat_id = message.get_chat_id();

        match parse_args(args) {
            Ok(val) => HandlerFuture::new(
                session
                    .set("counter", &val)
                    .and_then(move |()| api.execute(SendMessage::new(chat_id, "OK")))
                    .map(|_| HandlerResult::Stop),
            ),
            Err(err) => HandlerFuture::new(
                api.execute(SendMessage::new(chat_id, err.to_string()))
                    .map(|_| HandlerResult::Stop),
            ),
        }
    }

    fn expire(context: &mut Context, message: Message, args: Vec<String>) -> HandlerFuture {
        let session = context.get::<Session>().clone();
        let api = context.get::<Api>().clone();
        let chat_id = message.get_chat_id();

        match parse_args(args) {
            Ok(seconds) => HandlerFuture::new(
                session
                    .expire("counter", seconds)
                    .and_then(move |()| api.execute(SendMessage::new(chat_id, "OK")))
                    .map(|_| HandlerResult::Stop),
            ),
            Err(err) => HandlerFuture::new(
                api.execute(SendMessage::new(chat_id, err.to_string()))
                    .map(|_| HandlerResult::Stop),
            ),
        }
    }

    fn reset(context: &mut Context, message: Message, _args: Vec<String>) -> HandlerFuture {
        let session = context.get::<Session>().clone();
        let api = context.get::<Api>().clone();
        let chat_id = message.get_chat_id();
        HandlerFuture::new(
            session
                .del("counter")
                .and_then(move |()| api.execute(SendMessage::new(chat_id, "OK")))
                .map(|_| HandlerResult::Stop),
        )
    }

    fn increment(context: &mut Context, message: Message) -> HandlerFuture {
        let session = context.get::<Session>().clone();
        let api = context.get::<Api>().clone();
        let chat_id = message.get_chat_id();
        HandlerFuture::new(
            session
                .get::<usize>("counter")
                .and_then(move |val| {
                    let val = val.unwrap_or(0) + 1;
                    session.set("counter", &val).map(move |()| val)
                })
                .and_then(move |val| api.execute(SendMessage::new(chat_id, format!("Count: {}", val))))
                .map(|_| HandlerResult::Continue),
        )
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
        .add_handler("/set", set)
        .add_handler("/reset", reset)
        .add_handler("/expire", expire);
    tokio::run(
        create_store()
            .map_err(|e| log::error!("Failed to create session store: {:?}", e))
            .and_then(|store| {
                App::new()
                    .add_handler(FnHandler::from(log_message))
                    .add_handler(SessionHandler::new(store))
                    .add_handler(commands)
                    .add_handler(FnHandler::from(increment))
                    .run(api.clone(), UpdateMethod::poll(UpdatesStream::new(api)))
            }),
    );
}
