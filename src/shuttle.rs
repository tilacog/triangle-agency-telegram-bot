pub struct TelegramBot;

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for TelegramBot {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        Ok(())
    }
}

