use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    println!("Бот-модератор запущен...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // 1. Проверяем, есть ли вообще текст в сообщении
        if let Some(text) = msg.text() {
            
            // 2. Проверяем, совпадает ли текст с нашими командами
            if text == "-смс" || text == "!дел" {
                
                // 3. Смотрим, сделан ли реплай (ответ) на другое сообщение
                if let Some(replied_msg) = msg.reply_to_message() {
                    // Если реплай есть, удаляем ТО сообщение, на которое ответили
                    // Нам нужен ID чата и ID целевого сообщения
                    bot.delete_message(msg.chat.id, replied_msg.id)
                        .await?;
                    
                    // А теперь удаляем и само сообщение с командой ("-смс"), чтобы не мусорить
                    bot.delete_message(msg.chat.id, msg.id)
                        .await?;
                } else {
                    // Если реплая нет, удаляем только само сообщение с командой
                    bot.delete_message(msg.chat.id, msg.id)
                        .await?;
                }
            }
        }

        respond(())
    })
    .await;
}