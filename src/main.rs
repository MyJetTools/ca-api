use std::sync::Arc;

use app::AppContext;

mod app;
mod config_path;
mod flows;
mod http;
mod pem;
mod settings;
mod storage;

#[tokio::main]
async fn main() {
    let app = AppContext::new().await;
    let app = Arc::new(app);
    crate::http::start(&app);
    app.app_states.wait_until_shutdown().await;
}
