use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    pretty_env_logger::init();
    println!("Бот-модератор запущен...");

    let bot = Bot::from_env();
    
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(text) = msg.text() {
            if text == "-смс" || text == "!дел" {
                if let Some(replied_msg) = msg.reply_to_message() {
                    bot.delete_message(msg.chat.id, replied_msg.id)
                        .await?;
                    bot.delete_message(msg.chat.id, msg.id)
                        .await?;
                } else {
                    bot.delete_message(msg.chat.id, msg.id)
                        .await?;
                }
            }
        }

        respond(())
    })
    .await;
}
