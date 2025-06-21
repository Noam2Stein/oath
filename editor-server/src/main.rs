use oathc::*;

mod backend;
mod highlighting;
mod span;
mod watcher;
use backend::*;
use highlighting::*;
use span::*;

#[tokio::main]
async fn main() {
    run_server().await;
}
