use teloxide::prelude::*;

// Декларируем наши модули из соседних файлов
mod admin;
mod commands; 

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv().ok(); 
    println!("Бот запущен с разделением по модулям...");

    let bot = Bot::from_env();

    // Вызываем repl, но теперь указываем путь через имя модуля: commands::...
    commands::Command::repl(bot, commands::answer).await;
}