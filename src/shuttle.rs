use shuttle_runtime::SecretStore;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::task::JoinHandle;

pub struct TelegramBot {
    pub handle: JoinHandle<()>,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for TelegramBot {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        self.handle.await.map_err(|e| {
            shuttle_runtime::Error::from(std::io::Error::other(format!("Bot task failed: {}", e)))
        })
    }
}

pub async fn init(secrets: SecretStore) -> Result<TelegramBot, shuttle_runtime::Error> {
    tracing::info!("Starting bot");

    let token = secrets.get("TELOXIDE_TOKEN").ok_or_else(|| {
        shuttle_runtime::Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "TELOXIDE_TOKEN not found in secrets",
        ))
    })?;

    let bot = Bot::new(token);

    let handle = tokio::spawn(async move {
        Command::repl(bot, answer).await;
    });

    Ok(TelegramBot { handle })
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
            let outcome = crate::dice::roll();
            bot.send_message(msg.chat.id, outcome.to_string()).await?
        }
    };

    Ok(())
}

