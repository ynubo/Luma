use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use crate::admin;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "🛡️ Команды администрации:")]
pub enum Command {
    #[command(description = "Показать приветствие.")]
    Start,
    #[command(description = "Показать список команд.")]
    Help,
    #[command(description = "Выдать варн. Формат: /warn или ответом на сообщение.")]
    Warn(String),
    #[command(description = "Кикнуть пользователя. Формат: /kick или ответом.")]
    Kick(String),
    #[command(description = "Забанить пользователя. Формат: /ban или ответом.")]
    Ban(String),
    #[command(description = "Мут пользователя (ограничение отправки сообщений).")]
    Mute(String),
}

use teloxide::types::{ChatPermissions, UntilDate};

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    // Проверяем, что команда вызвана в группе или супергруппе, а не в ЛС у бота
    if msg.chat.is_private() {
        bot.send_message(msg.chat.id, "❌ Админские команды работают только в чатах группы!").await?;
        return Ok(());
    }

    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Бот запущен в группе. Используйте /help.").await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Warn(args) => {
            match resolve_target(&msg, &args) {
                Ok((_id, name)) => {
                    admin::give_warn(&name); // Логика варнов в нашем модуле
                    bot.send_message(msg.chat.id, format!("🛡️ Пользователю {} выдан варн.", name)).await?;
                }
                Err(err) => { bot.send_message(msg.chat.id, err).await?; }
            }
        }
        Command::Kick(args) => {
            match resolve_target(&msg, &args) {
                Ok((target_id, name)) => {
                    if target_id.0 == 0 {
                        bot.send_message(msg.chat.id, "⚠️ Для кика по юзернейму ответьте на его сообщение.").await?;
                        return Ok(());
                    }
                    // Кикаем: баним и сразу разбаниваем, чтобы пользователь мог зайти снова
                    bot.ban_chat_member(msg.chat.id, UserId(target_id.0 as u64)).await?;
                    bot.unban_chat_member(msg.chat.id, UserId(target_id.0 as u64)).await?;
                    bot.send_message(msg.chat.id, format!("💨 Пользователь {} исключен из чата (Kick).", name)).await?;
                }
                Err(err) => { bot.send_message(msg.chat.id, err).await?; }
            }
        }
        Command::Ban(args) => {
            match resolve_target(&msg, &args) {
                Ok((target_id, name)) => {
                    if target_id.0 == 0 {
                        bot.send_message(msg.chat.id, "⚠️ Для бана по юзернейму ответьте на его сообщение.").await?;
                        return Ok(());
                    }
                    // Перманентный бан
                    bot.ban_chat_member(msg.chat.id, UserId(target_id.0 as u64)).await?;
                    bot.send_message(msg.chat.id, format!("🚫 Пользователь {} навечно забанен (Ban).", name)).await?;
                }
                Err(err) => { bot.send_message(msg.chat.id, err).await?; }
            }
        }
        Command::Mute(args) => {
            match resolve_target(&msg, &args) {
                Ok((target_id, name)) => {
                    if target_id.0 == 0 {
                        bot.send_message(msg.chat.id, "⚠️ Для мута ответьте на сообщение пользователя.").await?;
                        return Ok(());
                    }
                    // Лишаем пользователя прав отправлять сообщения
                    let no_permissions = ChatPermissions::empty();
                    
                    bot.restrict_chat_member(msg.chat.id, UserId(target_id.0 as u64), no_permissions)
                        .await?;
                    
                    bot.send_message(msg.chat.id, format!("🔇 Пользователю {} запрещено писать в чат (Mute).", name)).await?;
                }
                Err(err) => { bot.send_message(msg.chat.id, err).await?; }
            }
        }
    };

    Ok(())
}

// Функция пытается определить ChatId и имя цели для наказания
fn resolve_target(msg: &Message, args: &str) -> Result<(ChatId, String), String> {
    // Вариант 1: Если это ответ на сообщение (Reply)
    if let Some(reply) = msg.reply_to_message() {
        if let Some(user) = reply.from() {
            let name = user.username.clone()
                .map(|u| format!("@{}", u))
                .unwrap_or_else(|| user.first_name.clone());
            return Ok((ChatId(user.id.0 as i64), name));
        }
    }

    // Вариант 2: Если передан аргумент (например, ID или никнейм текстом)
    if !args.is_empty() {
        // Если администратор ввел чистый ID числом
        if let Ok(id) = args.parse::<i64>() {
            return Ok((ChatId(id), format!("ID: {}", id)));
        }
        // Если передан юзернейм (обработка логики текстового юзернейма требует БД, 
        // пока запишем строку для вывода лога)
        return Ok((ChatId(0), args.to_string()));
    }

    Err(String::from("⚠️ Ошибка: ответьте на сообщение нарушителя или укажите его ID/Username после команды!"))
}