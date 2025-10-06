#![feature(iter_intersperse)]

pub mod dice;
mod shuttle;
mod telegram;

use shuttle::TelegramBot;
use shuttle_runtime::SecretStore;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> Result<TelegramBot, shuttle_runtime::Error> {
    shuttle::init(secrets).await
}
