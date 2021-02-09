use carapax::types::Command as RawCommand;
use carapax::{App, Client, Command, CommandMeta, ExecuteError, HandlerError, HandlerResult};
use dotenv::dotenv;
use std::convert::TryFrom;
use std::num::ParseIntError;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("{0}")]
    Execute(
        #[from]
        #[source]
        ExecuteError,
    ),
}

impl HandlerError for Error {
    fn result(&self) -> HandlerResult {
        HandlerResult::Stop
    }
}

#[derive(Debug)]
struct Start;

impl CommandMeta for Start {
    const NAME: &'static str = "/start";
}

impl From<RawCommand> for Start {
    fn from(_: RawCommand) -> Self {
        Self
    }
}

async fn handle_start(_start: Command<Start>, client: Client) -> Result<HandlerResult> {
    let msg = client.send_message("Hello!").reply_to_user().execute().await?;
    log::info!("sendMessage result: {:?}", msg);
    Ok(HandlerResult::Stop)
}

#[derive(Debug)]
struct UserId;

impl CommandMeta for UserId {
    const NAME: &'static str = "/user_id";
}

impl From<RawCommand> for UserId {
    fn from(_: RawCommand) -> Self {
        Self
    }
}

async fn handle_user_id(_user_id: Command<UserId>, client: Client) -> Result<HandlerResult> {
    let user_id = client
        .message()
        .get_user()
        .map(|user| user.id.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let msg = client.send_message(format!("Your ID is {}", user_id)).execute().await?;
    log::info!("sendMessage result: {:?}", msg);
    Ok(HandlerResult::Stop)
}

struct Sum {
    a: i64,
    b: i64,
}

impl CommandMeta for Sum {
    const NAME: &'static str = "/sum";
}

impl TryFrom<RawCommand> for Sum {
    type Error = SumError;

    fn try_from(cmd: RawCommand) -> Result<Self, Self::Error> {
        let args = cmd.get_args();
        let a = args.get(0).ok_or(SumError::NotEnoughArgs)?.parse()?;
        let b = args.get(1).ok_or(SumError::NotEnoughArgs)?.parse()?;
        Ok(Self { a, b })
    }
}

#[derive(Debug, thiserror::Error)]
enum SumError {
    #[error("{0}")]
    ParseInt(
        #[from]
        #[source]
        ParseIntError,
    ),
    #[error("Not enough arguments")]
    NotEnoughArgs,
}

async fn handle_sum(Command(Sum { a, b }): Command<Sum>, client: Client) -> Result<HandlerResult> {
    let sum = a + b;
    let msg = client.send_message(sum.to_string()).reply_to_user().execute().await?;
    log::info!("{:?}", msg);
    Ok(HandlerResult::Stop)
}

async fn handle_any(client: Client, command: RawCommand) -> Result<()> {
    let name = command.get_name();
    log::info!("handle {} command", name);
    client.send_message(format!("Got {} command", name)).execute().await?;
    Ok(())
}

#[derive(Debug)]
struct State {
    b: i32,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    App::from_env()
        .with_dispatcher(|dispatcher| {
            dispatcher
                .add_handler(handle_start)
                .add_handler(handle_user_id)
                .add_handler(handle_sum)
                .add_handler(handle_any);
        })
        .long_poll()
        .run()
        .await;
}
