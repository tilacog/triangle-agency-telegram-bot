#![feature(iter_intersperse)]

pub mod dice;

use shuttle_runtime::SecretStore;
use teloxide::{prelude::*, utils::command::BotCommands};

struct TelegramBot;

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for TelegramBot {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        Ok(())
    }
}

#[shuttle_runtime::main]
async fn init(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> Result<TelegramBot, shuttle_runtime::Error> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting bot");

    let token = secrets
        .get("TELOXIDE_TOKEN")
        .ok_or_else(|| shuttle_runtime::Error::Custom(
            shuttle_runtime::CustomError::new("TELOXIDE_TOKEN not found in secrets")
        ))?;

    let bot = Bot::new(token);

    tokio::spawn(async move {
        Command::repl(bot, answer).await;
    });

    Ok(TelegramBot)
}

/// These commands are supported:
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display this text.
    #[command(aliases = ["h", "?"])]
    Help,
    /// Roll 6d4.
    #[command(alias = "r")]
    Roll,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Roll => {
            let outcome = dice::roll();
            bot.send_message(msg.chat.id, outcome.to_string()).await?
        }
    };

    Ok(())
}
