use shuttle_runtime::SecretStore;
use teloxide::prelude::*;
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
    tracing::info!("Starting inline bot");

    let token = secrets.get("TELOXIDE_TOKEN").ok_or_else(|| {
        shuttle_runtime::Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "TELOXIDE_TOKEN not found in secrets",
        ))
    })?;

    let bot = Bot::new(token);

    let handle = tokio::spawn(async move {
        let handler = crate::telegram::create_handler();

        Dispatcher::builder(bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    });

    Ok(TelegramBot { handle })
}

