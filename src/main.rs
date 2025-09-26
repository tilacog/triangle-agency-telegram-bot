#![feature(iter_intersperse)]

pub mod dice;

use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting bot");

    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
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
