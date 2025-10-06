use tokio::task::JoinHandle;

pub struct TelegramBot {
    pub handle: JoinHandle<()>,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for TelegramBot {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        self.handle.await.map_err(|e| {
            shuttle_runtime::Error::from(std::io::Error::other(
                format!("Bot task failed: {}", e),
            ))
        })
    }
}

