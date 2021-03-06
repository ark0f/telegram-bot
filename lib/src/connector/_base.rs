use std::fmt::Debug;

use future::TelegramFuture;

use telegram_bot_fork_raw::{HttpRequest, HttpResponse};

/// Connector provides basic IO with Telegram Bot API server.
pub trait Connector: Debug {
    fn request(
        &self,
        url: Option<&str>,
        token: &str,
        req: HttpRequest,
    ) -> TelegramFuture<HttpResponse>;
}
