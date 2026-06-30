use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

// Подключаем модуль админки (он находится на одном уровне с нами в крейте)
use crate::admin;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
pub enum Command {
    #[command(description = "Показать приветственное сообщение.")]
    Start,
    #[command(description = "Показать список команд.")]
    Help,
    #[command(description = "Выдать варн игроку. Формат: /warn никнейм")]
    Warn(String),
}

// Делаем функцию публичной (pub), чтобы main.rs мог её вызвать
pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Привет! Бот Сатурн на связи. Используй /help.").await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Warn(username) => {
            if username.is_empty() {
                bot.send_message(msg.chat.id, "⚠️ Ошибка: укажи никнейм! Пример: /warn Magnus").await?;
            } else {
                admin::give_warn(&username);
                bot.send_message(msg.chat.id, format!("🛡️ Игроку {} успешно выдан варн.", username)).await?;
            }
        }
    };

    Ok(())
}