use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv().ok();
    println!("Бот готов к работе!");
    let bot = Bot::from_env();
}