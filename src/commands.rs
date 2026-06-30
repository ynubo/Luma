use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::types::ChatPermissions;

// Подключаем наш модуль админки
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
    #[command(description = "Мут пользователя. Формат: /mute или ответом.")]
    Mute(String),
}

// Асинхронная функция для проверки, является ли пользователь админом или создателем чата
async fn is_admin(bot: &Bot, chat_id: ChatId, user_id: UserId) -> bool {
    match bot.get_chat_member(chat_id, user_id).await {
        Ok(member) => {
            member.status().is_administrator() || member.status().is_owner()
        }
        Err(_) => false,
    }
}

// Функция-помощник для определения ChatId и имени цели (через reply или аргумент)
fn resolve_target(msg: &Message, args: &str) -> Result<(ChatId, String), String> {
    // 1. Вариант через ответ на сообщение (Reply)
    if let Some(reply) = msg.reply_to_message() {
        if let Some(user) = &reply.from {
            let name = user.username.clone()
                .map(|u| format!("@{}", u))
                .unwrap_or_else(|| user.first_name.clone());
            return Ok((ChatId(user.id.0 as i64), name));
        }
    }

    // 2. Вариант через аргументы в тексте (например, чистый ID)
    if !args.is_empty() {
        if let Ok(id) = args.parse::<i64>() {
            return Ok((ChatId(id), format!("ID: {}", id)));
        }
        return Ok((ChatId(0), args.to_string()));
    }

    Err(String::from("⚠️ Ошибка: ответьте на сообщение нарушителя или укажите его ID после команды!"))
}

// Главный публичный обработчик команд
pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    // Проверяем, что команды выполняются в группе
    if msg.chat.is_private() {
        bot.send_message(msg.chat.id, "❌ Админские команды работают только в чатах группы!").await?;
        return Ok(());
    }

    // Вытаскиваем отправителя сообщения
    let sender = match msg.from() {
        Some(user) => user,
        None => return Ok(()),
    };

    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Бот Сатурн запущен в группе. Используйте /help.").await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        // Для всех остальных команд (Warn, Kick, Ban, Mute) включаем проверку прав
        _ => {
            if !is_admin(&bot, msg.chat.id, sender.id).await {
                bot.send_message(msg.chat.id, "❌ Ошибка: у вас нет прав администратора для этой команды!").await?;
                return Ok(());
            }

            // Если проверку на админа прошли, матчим конкретное действие
            match cmd {
                Command::Warn(args) => {
                    match resolve_target(&msg, &args) {
                        Ok((_id, name)) => {
                            admin::give_warn(&name);
                            bot.send_message(msg.chat.id, format!("🛡️ Пользователю {} выдан варн.", name)).await?;
                        }
                        Err(err) => { bot.send_message(msg.chat.id, err).await?; }
                    }
                }
                Command::Kick(args) => {
                    match resolve_target(&msg, &args) {
                        Ok((target_id, name)) => {
                            if target_id.0 == 0 {
                                bot.send_message(msg.chat.id, "⚠️ Для кика по юзернейму ответьте на сообщение пользователя.").await?;
                                return Ok(());
                            }
                            bot.ban_chat_member(msg.chat.id, UserId(target_id.0 as u64)).await?;
                            bot.unban_chat_member(msg.chat.id, UserId(target_id.0 as u64)).await?;
                            bot.send_message(msg.chat.id, format!("💨 Пользователь {} исключен из чата.", name)).await?;
                        }
                        Err(err) => { bot.send_message(msg.chat.id, err).await?; }
                    }
                }
                Command::Ban(args) => {
                    match resolve_target(&msg, &args) {
                        Ok((target_id, name)) => {
                            if target_id.0 == 0 {
                                bot.send_message(msg.chat.id, "⚠️ Для бана по юзернейму ответьте на сообщение пользователя.").await?;
                                return Ok(());
                            }
                            bot.ban_chat_member(msg.chat.id, UserId(target_id.0 as u64)).await?;
                            bot.send_message(msg.chat.id, format!("🚫 Пользователь {} навечно забанен.", name)).await?;
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
                            let no_permissions = ChatPermissions::empty();
                            bot.restrict_chat_member(msg.chat.id, UserId(target_id.0 as u64), no_permissions).await?;
                            bot.send_message(msg.chat.id, format!("🔇 Пользователю {} запрещено писать в чат.", name)).await?;
                        }
                        Err(err) => { bot.send_message(msg.chat.id, err).await?; }
                    }
                }
                _ => {} 
            }
        }
    };

    Ok(())
}